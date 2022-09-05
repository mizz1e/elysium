//#![deny(warnings)]
#![feature(abi_thiscall)]
#![feature(bound_map)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(mem_copy_fn)]
#![feature(pointer_byte_offsets)]
#![feature(result_option_inspect)]
#![feature(sync_unsafe_cell)]
#![feature(strict_provenance)]

use elysium_sdk::material::{Material, MaterialKind};
use elysium_sdk::{Interfaces, LibraryKind, Vars, Vdf};
use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};
use std::io::Write;
use std::path::Path;
use std::{mem, ptr, thread};

pub use controls::Controls;
pub use menu::Menu;
pub use networked::Networked;
pub use scene::Scene;
pub use state::State;

mod controls;
mod menu;
mod scene;

pub mod anti_aim;
pub mod assets;
pub mod entity;
pub mod hooks;
pub mod library;
pub mod networked;
pub mod pattern;
//pub mod simulation;
pub mod state;

// this is called by glibc after the library is loaded into a process
#[link_section = ".init_array"]
#[used]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

#[link_section = ".text.startup"]
unsafe extern "C" fn bootstrap() {
    let program = std::env::args_os().next();
    let is_csgo = program.as_deref().map(Path::new).map_or(false, |path| {
        path.ends_with("csgo_linux64") || path.ends_with("csgo-launcher")
    });

    if is_csgo {
        thread::spawn(main);
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
fn main() {
    unsafe {
        library::wait_for_serverbrowser();

        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        thread::spawn(console);

        let interfaces = state.interfaces.as_ref().unwrap_unchecked();
        let console = &interfaces.console;
        let client = &interfaces.client;
        let model_render = &interfaces.model_render;
        let material_system = &interfaces.material_system;

        let globals = &mut *(client.globals() as *mut _);
        let input = &mut *(client.input() as *mut _);

        println!("{globals:?}");
        println!("{input:?}");

        console.write(format_args!("welcome to elysium\n"));

        let vars = Vars::from_loader(|kind| {
            let name = kind.name();
            let var = console.var(name);

            if var.is_none() {
                println!(
                    "elysium | config variable \x1b[38;5;2m{name}\x1b[m was not found, remove it"
                );

                ptr::null_mut()
            } else {
                println!("elysium | config variable \x1b[38;5;2m{name}\x1b[m found at \x1b[38;5;3m{address:?}\x1b[m");

                var as *const _ as _
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

        state.hooks.vdf_from_bytes = Some(mem::transmute(new));

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

        let glx = link::load_module("libGL.so.1").expect("libGL.so.1");

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

        println!("create gold");
        state.materials.gold = create_material();
    }
}

fn create_material() -> Option<&'static Material> {
    let material = MaterialKind::Glow;
    let vdf = Vdf::from_bytes(material.base(), material.vdf().unwrap())?;
    let material = material_system.create(material.name(), vdf)?;

    println!("name = {:?}", material.name());
    println!("group = {:?}", material.group());

    Some(material)
}
