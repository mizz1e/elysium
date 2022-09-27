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

use elysium_sdk::material::{Material, MaterialFlag, MaterialKind};
use elysium_sdk::{Interfaces, LibraryKind, Vars, Vdf};
use error::Error;
use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};
use std::borrow::Cow;
use std::ffi::{CStr, CString, OsString};
use std::io::Write;
use std::os::unix::ffi::OsStringExt;
use std::thread;
use std::time::Duration;
use std::{env, ffi, iter, mem, ptr};

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
pub mod library;
pub mod networked;
pub mod pattern;
pub mod state;

type Main = unsafe extern "C" fn(argc: libc::c_int, argv: *const *const libc::c_char);

const LAUNCHER_CLIENT: &str = "bin/linux64/launcher_client.so";
const LAUNCHER_MAIN: &str = "LauncherMain";

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

mod launcher {
    use super::const_cstr;
    use std::borrow::Cow;
    use std::ffi::CStr;

    const CONNECT: Cow<'static, CStr> = const_cstr("+connect\0");
    const MAP: Cow<'static, CStr> = const_cstr("+map\0");
    pub const NO_BREAKPAD: Cow<'static, CStr> = const_cstr("-nobreakpad\0");
    pub const FPS: Cow<'static, CStr> = const_cstr("+fps_max\0");
    pub const FPS_144: Cow<'static, CStr> = const_cstr("144\0");
}

/// X11 DISPLAY sanity check as CSGO prefers to segmentation fault.
#[inline]
fn check_display() -> Result<(), Error> {
    env::var_os("DISPLAY").ok_or(Error::NoDisplay)?;

    Ok(())
}

#[inline]
fn run() -> Result<(), Error> {
    let options = Options::parse();

    check_display()?;

    let args = env::args_os()
        .map(cstring_from_osstring)
        .chain(iter::once(Ok(launcher::FPS)))
        .chain(iter::once(Ok(launcher::FPS_144)))
        .chain(iter::once(Ok(launcher::NO_BREAKPAD)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::InvalidArgs)?;

    let args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .chain(iter::once(ptr::null()))
        .collect::<Vec<_>>();

    let launcher = unsafe { link::load_module(LAUNCHER_CLIENT).map_err(Error::LoadLauncher)? };

    let main: Main = unsafe {
        let address = launcher.symbol(LAUNCHER_MAIN).map_err(Error::FindMain)?;

        mem::transmute(address.symbol.address)
    };

    let pretty_args = env::args_os()
        .map(cow_str_from_debug)
        .intersperse(Cow::Borrowed(", "))
        .collect::<String>();

    println!("\x1b[38;5;2minfo:\x1b[m starting csgo with args: {pretty_args}");

    std::thread::spawn(main2);

    unsafe {
        (main)(args.len().saturating_sub(1) as _, args.as_ptr());
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("\x1b[38;5;1merror:\x1b[m {error}");
    }
}

#[inline]
fn hooked(name: &str) {
    println!("elysium | hooked \x1b[38;5;2m{name}\x1b[m");
}

#[inline]
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

#[inline]
fn main2() {
    while !{
        link::is_module_loaded("libGLX.so.0.0.0")
            && link::is_module_loaded("libSDL2-2.0.so.0")
            && link::is_module_loaded("vgui2_client.so")
    } {
        thread::sleep(Duration::from_millis(100));
    }

    unsafe {
        let state = State::get();

        let glx = link::load_module("libGLX.so.0.0.0").expect("libGL.so.0.0.0");
        let address = glx
            .symbol("glXGetProcAddress")
            .expect("glXGetProcAddress")
            .symbol
            .address;

        state.proc_address = mem::transmute(address);

        let sdl = link::load_module("libSDL2-2.0.so.0").expect("libSDL2-2.0.so.0");

        let address = sdl
            .symbol("SDL_GL_SwapWindow")
            .expect("SDL_GL_SwapWindow")
            .symbol
            .address;

        let swap_window =
            elysium_mem::next_abs_addr_mut_ptr::<SwapWindow>(address as _).expect("swap_window");

        state.hooks.swap_window = Some(swap_window.replace(hooks::swap_window));

        hooked("SDL_GL_SwapWindow");

        let address = sdl
            .symbol("SDL_PollEvent")
            .expect("SDL_PollEvent")
            .symbol
            .address;

        let poll_event =
            elysium_mem::next_abs_addr_mut_ptr::<PollEvent>(address as _).expect("poll_event");

        state.hooks.poll_event = Some(poll_event.replace(hooks::poll_event));

        hooked("SDL_PollEvent");

        while !link::is_module_loaded("serverbrowser_client.so") {
            thread::sleep(Duration::from_millis(100));
        }
    }

    unsafe {
        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        thread::spawn(console);

        let interfaces = state.interfaces.as_mut().unwrap_unchecked();
        let console = &mut interfaces.console;
        let client = &interfaces.client;
        let model_render = &interfaces.model_render;
        let material_system = &interfaces.material_system;

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

        let bytes = pattern::get(LibraryKind::Client, &pattern::VDF_FROM_BYTES).unwrap();
        let base = bytes.as_ptr().cast::<i32>().byte_add(1);
        let new = base.byte_add(4).byte_offset(base.read() as isize);

        Vdf::set_from_bytes(mem::transmute(new));

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
