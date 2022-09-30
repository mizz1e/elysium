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
use elysium_sdk::material::{Material, Materials};
use elysium_sdk::{Interface, InterfaceKind, Interfaces, LibraryKind, Vars, Vdf};
use error::Error;
use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};
use std::borrow::Cow;
use std::ffi::{CStr, CString, OsString};
use std::io::Write;
use std::os::unix::ffi::OsStringExt;
use std::thread;
use std::time::Duration;
use std::{env, ffi, mem, ptr};

pub use networked::Networked;
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
pub mod networked;
pub mod pattern;
pub mod state;

type Main = unsafe extern "C" fn(argc: libc::c_int, argv: *const *const libc::c_char);

const fn const_cstr(string: &'static str) -> Cow<'static, CStr> {
    unsafe { Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(string.as_bytes())) }
}

fn cstring_from_osstring(string: OsString) -> Result<Cow<'static, CStr>, ffi::NulError> {
    let bytes = string.into_vec();
    let string = CString::new(bytes)?;

    Ok(Cow::Owned(string))
}

fn cow_str_from_debug(string: OsString) -> Cow<'static, str> {
    Cow::Owned(format!("{string:?}"))
}

/// X11 DISPLAY sanity check as CSGO prefers to segmentation fault.
fn check_display() -> Result<(), Error> {
    env::var_os("DISPLAY").ok_or(Error::NoDisplay)?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("\x1b[38;5;1merror:\x1b[m {error}");
    }
}

fn run() -> Result<(), Error> {
    let options = Options::parse();

    check_display()?;

    if options.i_agree_to_be_banned {
        thread::spawn(setup_hooks);
    }

    launcher::launch(options)?;

    Ok(())
}

fn hooked(name: &str) {
    println!("elysium | hooked \x1b[38;5;2m{name}\x1b[m");
}

fn console() {
    let state = State::get();
    let Interfaces { engine, .. } = state.interfaces.as_ref().unwrap();

    let mut out = std::io::stdout();
    let mut lines = std::io::stdin().lines().flatten();

    while let Some(line) = {
        let _ = write!(out, "> ");
        let _ = out.flush();

        lines.next()
    } {
        engine.execute_command(line, true);
    }
}

fn is_sdl_loaded() -> bool {
    let sdl = link::is_module_loaded("libSDL2-2.0.so.0");

    // SDL may not be initialized yet, wait for VGUI to initialize it.
    let vgui = link::is_module_loaded("vgui2_client.so");

    sdl && vgui
}

fn is_materials_loaded() -> bool {
    let materials = link::is_module_loaded("materialsystem_client.so");

    // Client contains `Vdf::from_bytes`, which is needed to create materials.
    let client = link::is_module_loaded("client_client.so");

    materials && client
}

fn is_browser_loaded() -> bool {
    let browser = link::is_module_loaded("serverbrowser_client.so");

    browser
}

fn sleep_until<F>(f: F)
where
    F: Fn() -> bool,
{
    while !f() {
        thread::sleep(Duration::from_millis(100));
    }
}

fn setup_hooks() {
    sleep_until(is_sdl_loaded);

    let state = State::get();

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

    hooked("SDL_GL_SwapWindow");

    let address = sdl
        .symbol("SDL_PollEvent")
        .expect("SDL_PollEvent")
        .symbol
        .address;

    let poll_event = unsafe {
        elysium_mem::next_abs_addr_mut_ptr::<PollEvent>(address as _).expect("poll_event")
    };

    state.hooks.poll_event = Some(unsafe { poll_event.replace(hooks::poll_event) });

    hooked("SDL_PollEvent");

    sleep_until(is_materials_loaded);

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

    let glow = materials.from_kind(material::Kind::Glow).unwrap();

    state::material::DECAL.store(Some(glow));

    unsafe {
        materials.hook_create(hooks::create_material);
        materials.hook_find(hooks::find_material);
    }

    sleep_until(is_browser_loaded);

    unsafe {
        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        thread::spawn(console);

        let interfaces = state.interfaces.as_mut().unwrap_unchecked();

        let show = interfaces.game_console.show_address();

        unsafe extern "C" fn show_hook(this: *const ()) {
            println!("show console");
        }

        elysium_mem::unprotect(show, |show, prot| {
            show.replace(show_hook as *const ());
            hooked("GameConsole::Show");
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
        state.networked.update(client);

        /*let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_LAYERS).unwrap();
        let _animation_layers = bytes.as_ptr().byte_add(35).cast::<u32>().read();

        let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_STATE).unwrap();
        let _animation_state = bytes.as_ptr().byte_add(52).cast::<u32>().read();*/

        let bytes = pattern::get(LibraryKind::Engine, &pattern::CL_MOVE).unwrap();

        state.hooks.cl_move = Some(mem::transmute(bytes.as_ptr()));

        let address = client.create_move_address().cast::<CreateMove>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.create_move = Some(address.replace(hooks::create_move));
            hooked("CreateMove");
            prot
        });

        let address = model_render.draw_model_address().cast::<DrawModel>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.draw_model = Some(address.replace(hooks::draw_model));
            hooked("DrawModelExecute");
            prot
        });

        let address = client
            .frame_stage_notify_address()
            .cast::<FrameStageNotify>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.frame_stage_notify = Some(address.replace(hooks::frame_stage_notify));
            hooked("FrameStageNotify");
            prot
        });

        let address = client.override_view_address().cast::<OverrideView>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.override_view = Some(address.replace(hooks::override_view));
            hooked("OverrideView");
            prot
        });
    }
}
