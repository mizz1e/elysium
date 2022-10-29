//#![deny(warnings)]
#![allow(dead_code)]
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
use std::time::Instant;
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
    util::check_linker_path(&csgo_dir)?;

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

use libloading::Library;

fn glx() -> Option<()> {
    log::trace!("finding glx proc address");

    let glx = unsafe { Library::new("libGLX.so").ok()? };
    let proc_addr = unsafe { *glx.get(b"glXGetProcAddress\0").ok()? };

    log::trace!("found glx proc address");

    State::get().proc_address = proc_addr;

    Some(())
}

fn sdl() -> Option<()> {
    log::trace!("finding sdl symbols");

    let glx = unsafe { Library::new("libSDL2-2.0.so.0").ok()? };
    let poll_event = unsafe { *glx.get(b"SDL_PollEvent\0").ok()? };
    let swap_window = unsafe { *glx.get(b"SDL_GL_SwapWindow\0").ok()? };

    log::trace!("found sdl symbols, finding original methods...");

    let swap_window = unsafe { elysium_mem::next_abs_addr_mut_ptr::<SwapWindow>(swap_window)? };
    let poll_event = unsafe { elysium_mem::next_abs_addr_mut_ptr::<PollEvent>(poll_event)? };

    log::trace!("found sdl original methods, hooking...");

    let state = State::get();

    state.hooks.swap_window = Some(unsafe { swap_window.replace(hooks::swap_window) });
    state.hooks.poll_event = Some(unsafe { poll_event.replace(hooks::poll_event) });

    Some(())
}

fn setup() -> Result<(), Error> {
    let now = Instant::now();

    // Wait for SDL to load before hooking.
    util::sleep_until(util::is_sdl_loaded);

    log::trace!("sdl loaded. took {:?}", now.elapsed());

    let state = State::get();

    // TODO: Clean this mess up.
    use std::collections::HashSet;
    state.world = Some(HashSet::new());
    state.blur = Some(HashSet::new());
    state.blur_static = Some(HashSet::new());
    state.init_time = Some(std::time::Instant::now());

    glx();
    sdl();

    unsafe {
        let module = link::load_module("client_client.so").unwrap();
        let bytes = module.bytes();
        let opcode = &pattern::VDF_FROM_BYTES.find(bytes).unwrap().1[..5];

        log::trace!("vdf from_bytes = {opcode:02X?}");

        let ip = opcode.as_ptr().byte_add(1);
        let reladdr = ip.cast::<i32>().read() as isize;
        let absaddr = ip.byte_add(4).byte_offset(reladdr);

        Vdf::set_from_bytes(mem::transmute(absaddr));
    }

    util::sleep_until(util::is_materials_loaded);

    log::trace!("material system loaded. took {:?}", now.elapsed());

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

    log::trace!("server browser loaded. took {:?}", now.elapsed());

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

                    log::info!("convar {name} found at {pointer:?}");

                    pointer
                }
                None => {
                    log::info!("convar {name} missing :warning:");

                    ptr::null_mut()
                }
            }
        });

        state.globals = Some(globals);
        state.input = Some(input);
        state.vars = vars.ok();

        networked::init(&client);

        let address = client.create_move_address().cast::<CreateMove>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.create_move = Some(address.replace(hooks::create_move));
            log::info!("hooked clientmode createmove");
            prot
        });

        let address = model_render.draw_model_address().cast::<DrawModel>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.draw_model = Some(address.replace(hooks::draw_model));
            log::info!("hooked modelrender dme");
            prot
        });

        let address = client
            .frame_stage_notify_address()
            .cast::<FrameStageNotify>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.frame_stage_notify = Some(address.replace(hooks::frame_stage_notify));
            log::info!("hooked client fsn");
            prot
        });

        let address = client.override_view_address().cast::<OverrideView>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.override_view = Some(address.replace(hooks::override_view));
            log::info!("hooked clientmode overrideview");
            prot
        });
    }

    Ok(())
}
