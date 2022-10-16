//#![deny(warnings)]
#![feature(abi_thiscall)]
#![feature(bound_map)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(mem_copy_fn)]
#![feature(iter_intersperse)]
#![feature(pointer_byte_offsets)]
#![feature(result_option_inspect)]
#![feature(sync_unsafe_cell)]
#![feature(strict_provenance)]

use elysium_sdk::material;
use elysium_sdk::material::Materials;
use elysium_sdk::networked;
use elysium_sdk::{Interface, InterfaceKind, LibraryKind, Vars, Vdf};
use error::Error;
use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, mem, ptr, thread};

pub use options::Options;
pub use state::State;

mod error;
mod options;
mod ui;

pub mod anti_aim;
pub mod assets;
pub mod entity;
pub mod hooks;
pub mod launcher;
pub mod library;
pub mod pattern;
pub mod state;
pub mod util;

const fn const_cstr(string: &'static str) -> Cow<'static, CStr> {
    unsafe { Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(string.as_bytes())) }
}

fn main() {
    env_logger::init();

    if let Err(error) = run() {
        log::error!("failed to even run: {error}");
    }
}

fn run() -> Result<(), Error> {
    // CLI options, duh!
    let options = Options::parse();

    // Why even try to continue if you don't even have the game?
    // Also, we *really* need this path for below.
    let csgo_dir = util::determine_csgo_dir().ok_or(Error::NoCsgo)?;

    // Automatically append "LD_LIBRARY_PATH" otherwise CSGO can't find any libraries!
    if env::var_os("FUCK_LINKER_PATH").is_none() {
        let current_exe = env::current_exe().map_err(|_| Error::NoCsgo)?;
        let mut linker_path = util::var_path("LD_LIBRARY_PATH");

        linker_path.push(csgo_dir.join("bin/linux64"));

        let linker_path = env::join_paths(linker_path).unwrap_or_default();

        Command::new(current_exe)
            .args(env::args_os().skip(1))
            .current_dir(csgo_dir)
            .env("FUCK_LINKER_PATH", ":thumbs_up:")
            .env("LD_LIBRARY_PATH", linker_path)
            .exec();
    }

    // X11 `DISPLAY` sanity check as CSGO prefers to segmentation fault.
    //
    // In the future, this shouldn't be needed anymore (what v2 aims to implement).
    env::var_os("DISPLAY").ok_or(Error::NoDisplay)?;

    // Spawn a seperate thread in order to hook functions in modules.
    thread::spawn(setup);

    // Actually launch the game (bin/linux64/launcher_client.so).
    launcher::launch(options)?;

    Ok(())
}

fn setup() -> Result<(), Error> {
    // Wait for SDL to load before hooking.
    util::sleep_until(util::is_sdl_loaded);

    let state = State::get();

    // TODO: Clean this mess up.
    use std::collections::HashSet;
    state.world = Some(HashSet::new());
    state.blur = Some(HashSet::new());
    state.blur_static = Some(HashSet::new());
    state.init_time = Some(std::time::Instant::now());

    let glx = unsafe { link::load_module("libGLX.so.0.0.0").expect("libGL.so.0.0.0") };
    let address = glx
        .symbol("glXGetProcAddress")
        .expect("glXGetProcAddress")
        .symbol
        .address;

    state.proc_address = unsafe { mem::transmute(address) };

    let sdl = unsafe { link::load_module("libSDL2-2.0.so.0").expect("libSDL2-2.0.so.0") };
    let address = sdl
        .symbol("SDL_GL_SwapWindow")
        .expect("SDL_GL_SwapWindow")
        .symbol
        .address;

    let swap_window = unsafe {
        elysium_mem::next_abs_addr_mut_ptr::<SwapWindow>(address as _).expect("swap_window")
    };

    state.hooks.swap_window = Some(unsafe { swap_window.replace(hooks::swap_window) });

    let address = sdl
        .symbol("SDL_PollEvent")
        .expect("SDL_PollEvent")
        .symbol
        .address;

    let poll_event = unsafe {
        elysium_mem::next_abs_addr_mut_ptr::<PollEvent>(address as _).expect("poll_event")
    };

    state.hooks.poll_event = Some(unsafe { poll_event.replace(hooks::poll_event) });

    util::sleep_until(util::is_materials_loaded);

    println!("materials loaded");

    let kind = InterfaceKind::Materials;
    let path = kind.library().path();
    let name = kind.name();
    let module = unsafe { link::load_module(path).expect("materials") };
    let address = module
        .symbol("s_pInterfaceRegs")
        .expect("interface registry")
        .symbol
        .address as *const *const Interface;

    let interfaces = unsafe { &**address };
    let interface = interfaces.get(name);
    let materials = unsafe { &mut *(interface as *mut Materials) };

    unsafe {
        let bytes = pattern::get(LibraryKind::Client, &pattern::VDF_FROM_BYTES).unwrap();
        let base = bytes.as_ptr().cast::<i32>().byte_add(1);
        let new = base.byte_add(4).byte_offset(base.read() as isize);

        Vdf::set_from_bytes(mem::transmute(new));
    }

    unsafe {
        // load what we need
        materials.init();
    }

    let blood = materials
        .from_kind("elysium/blood\0", material::Kind::Glow)
        .unwrap();

    let decal = materials
        .from_kind("elysium/decal\0", material::Kind::Glow)
        .unwrap();

    let fire = materials
        .from_kind("elysium/fire\0", material::Kind::Glow)
        .unwrap();

    let impact = materials
        .from_kind("elysium/impact\0", material::Kind::Glow)
        .unwrap();

    let muzzle_flash = materials
        .from_kind("elysium/muzzle_flash\0", material::Kind::Glow)
        .unwrap();

    let path = materials
        .from_kind("elysium/path\0", material::Kind::Glow)
        .unwrap();

    let particle = materials
        .from_kind("elysium/particle\0", material::Kind::Glow)
        .unwrap();

    let prop = materials
        .from_kind("elysium/prop\0", material::Kind::Glow)
        .unwrap();

    let smoke = materials
        .from_kind("elysium/smoke\0", material::Kind::Glow)
        .unwrap();

    let tree = materials
        .from_kind("elysium/tree\0", material::Kind::Glow)
        .unwrap();

    let flat = materials
        .from_kind("elysium/flat\0", material::Kind::Flat)
        .unwrap();

    let glow = materials
        .from_kind("elysium/glow\0", material::Kind::Glow)
        .unwrap();

    state::material::BLOOD.store(Some(blood));
    state::material::DECAL.store(Some(decal));
    state::material::FIRE.store(Some(fire));
    state::material::MUZZLE_FLASH.store(Some(muzzle_flash));
    state::material::PATH.store(Some(path));
    state::material::IMPACT.store(Some(impact));
    state::material::PARTICLE.store(Some(particle));
    state::material::PROP.store(Some(prop));
    state::material::SMOKE.store(Some(smoke));
    state::material::TREE.store(Some(tree));

    state::material::FLAT.store(Some(flat));
    state::material::GLOW.store(Some(glow));

    unsafe {
        materials.hook_create(hooks::create_material);
        materials.hook_find(hooks::find_material);
    }

    util::sleep_until(util::is_browser_loaded);
    println!("browser loaded");

    unsafe {
        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        let interfaces = state.interfaces.as_mut().unwrap_unchecked();

        let show = interfaces.game_console.show_address();

        unsafe extern "C" fn show_hook(this: *const ()) {
            println!("show console");
        }

        elysium_mem::unprotect(show, |show, prot| {
            show.replace(show_hook as *const ());
            prot
        });

        let console = &mut interfaces.console;

        println!("{console:?}");

        let client = &interfaces.client;
        let model_render = &interfaces.model_render;
        let materials = &interfaces.materials;

        let globals = &mut *(client.globals() as *mut _);
        let input = &mut *(client.input() as *mut _);

        console.write(format_args!("welcome to elysium\n")).unwrap();

        let vars = Vars::from_loader(|kind| {
            let name = kind.name();
            let var = console.var::<_, i32>(name);

            match var {
                Some(var) => {
                    let pointer = var as *const _ as _;

                    println!("convar {name} found at {pointer:?}");

                    pointer
                }
                None => {
                    println!("convar {name} missing :warning:");

                    ptr::null_mut()
                }
            }
        });

        println!("{vars:?}");

        state.globals = Some(globals);
        state.input = Some(input);
        state.vars = vars.ok();

        networked::init(&client);

        /*let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_LAYERS).unwrap();
        let _animation_layers = bytes.as_ptr().byte_add(35).cast::<u32>().read();

        let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_STATE).unwrap();
        let _animation_state = bytes.as_ptr().byte_add(52).cast::<u32>().read();*/

        let bytes = pattern::get(LibraryKind::Engine, &pattern::CL_MOVE).unwrap();

        state.hooks.cl_move = Some(mem::transmute(bytes.as_ptr()));

        let address = client.create_move_address().cast::<CreateMove>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.create_move = Some(address.replace(hooks::create_move));
            prot
        });

        let address = model_render.draw_model_address().cast::<DrawModel>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.draw_model = Some(address.replace(hooks::draw_model));
            prot
        });

        let address = client
            .frame_stage_notify_address()
            .cast::<FrameStageNotify>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.frame_stage_notify = Some(address.replace(hooks::frame_stage_notify));
            prot
        });

        let address = client.override_view_address().cast::<OverrideView>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.override_view = Some(address.replace(hooks::override_view));
            prot
        });
    }

    Ok(())
}
