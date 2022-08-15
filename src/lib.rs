//#![deny(warnings)]
#![feature(abi_thiscall)]
#![feature(const_ptr_offset_from)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(sync_unsafe_cell)]
#![feature(strict_provenance)]
// todo remove
#![feature(const_maybe_uninit_zeroed)]

use elysium_sdk::{LibraryKind, Vars};
use state::Hooks;
use std::path::Path;
use std::{mem, thread};

pub use controls::Controls;
pub use entity::Entity;
pub use menu::Menu;
pub use networked::Networked;
pub use scene::Scene;
pub use state::State;

mod controls;
mod entity;
mod menu;
mod scene;

pub mod assets;
pub mod hooks;
pub mod library;
pub mod networked;
pub mod pattern;
pub mod state;

// this is called by glibc after the library is loaded into a process
#[link_section = ".init_array"]
#[used]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

#[link_section = ".text.startup"]
unsafe extern "C" fn bootstrap() {
    // check the name of the process we're injected into
    let is_csgo = std::env::args()
        .next()
        .and_then(|path| {
            let path = Path::new(&path);
            let name = path.file_name()?;
            let name = name.to_str()?;
            let is_csgo = matches!(name, "csgo_linux64" | "csgo-launcher");

            is_csgo.then(|| true)
        })
        .unwrap_or(false);

    // bail if we're injected into not csgo
    if !is_csgo {
        return;
    }

    // spawn a new thread to prevent blocking csgo
    thread::spawn(main);
}

#[inline]
fn hooked(name: &str) {
    println!("elysium | hooked \x1b[38;5;2m{name}\x1b[m");
}

#[inline]
fn main() {
    unsafe {
        library::wait_for_serverbrowser();

        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        let interfaces = state.interfaces.as_ref().unwrap_unchecked();
        let console = &interfaces.console;
        let client = &interfaces.client;
        let model_render = &interfaces.model_render;
        let material_system = &interfaces.material_system;

        let globals = &mut *(client.globals() as *mut _);
        let input = &mut *(client.input() as *mut _);

        println!("{globals:?}");
        println!("{input:?}");

        console.write("welcome to elysium\n");

        let vars = Vars::from_loader(|var_kind| {
            let name = var_kind.as_str();
            let cstr = var_kind.as_nul_str();
            let address = console.var(cstr);

            if address.is_null() {
                println!(
                    "elysium | config variable \x1b[38;5;2m{name}\x1b[m was not found, remove it"
                );
            } else {
                println!("elysium | config variable \x1b[38;5;2m{name}\x1b[m found at \x1b[38;5;3m{address:?}\x1b[m");
            }

            address
        });

        state.globals = Some(globals);
        state.input = Some(input);
        state.vars = Some(vars);
        state.networked.update(client);

        /*let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_LAYERS).unwrap();
        let _animation_layers = bytes.as_ptr().byte_add(35).cast::<u32>().read();

        let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_STATE).unwrap();
        let _animation_state = bytes.as_ptr().byte_add(52).cast::<u32>().read();*/

        // TODO: clean this up (remove maybeuninit).
        use std::mem::MaybeUninit;
        let mut hooks = MaybeUninit::<Hooks>::uninit();
        let hooks_ref = hooks.as_mut_ptr();

        let bytes = pattern::get(LibraryKind::Engine, &pattern::CL_MOVE).unwrap();
        (*hooks_ref).cl_move = mem::transmute(bytes.as_ptr());

        let bytes = pattern::get(LibraryKind::Client, &pattern::VDF_FROM_BYTES).unwrap();
        // TODO: make not shit
        let base = bytes.as_ptr().cast::<i32>().byte_add(1);
        let new = base.byte_add(4).byte_offset(base.read() as isize);
        (*hooks_ref).vdf_from_bytes = mem::transmute(new);

        use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};

        let ptr = client.create_move_address().cast::<CreateMove>();

        elysium_mem::unprotect(ptr, |ptr, prot| {
            (*hooks_ref).create_move = ptr.replace(hooks::create_move);
            hooked("CreateMove");
            prot
        });

        let ptr = model_render.draw_model_address().cast::<DrawModel>();

        elysium_mem::unprotect(ptr, |ptr, prot| {
            (*hooks_ref).draw_model = ptr.replace(hooks::draw_model);
            hooked("DrawModelExecute");
            prot
        });

        let ptr = client
            .frame_stage_notify_address()
            .cast::<FrameStageNotify>();

        elysium_mem::unprotect(ptr, |ptr, prot| {
            (*hooks_ref).frame_stage_notify = ptr.replace(hooks::frame_stage_notify);
            hooked("FrameStageNotify");
            prot
        });

        let ptr = client.override_view_address().cast::<OverrideView>();

        elysium_mem::unprotect(ptr, |ptr, prot| {
            (*hooks_ref).override_view = ptr.replace(hooks::override_view);
            hooked("OverrideView");
            prot
        });

        let glx = link::Library::load("libGL.so.1").unwrap();

        state.get_proc_address =
            mem::transmute(glx.symbol_ptr::<_, u8>("glXGetProcAddress").unwrap());

        let sdl = link::Library::load("libSDL2-2.0.so.0").unwrap();

        let swap_window: *const SwapWindow = sdl.symbol_ptr("SDL_GL_SwapWindow").unwrap();
        let swap_window = elysium_mem::next_abs_addr_mut(swap_window as *mut SwapWindow);
        (*hooks_ref).swap_window = swap_window.replace(hooks::swap_window);

        hooked("SDL_GL_SwapWindow");

        let poll_event: *const PollEvent = sdl.symbol_ptr("SDL_PollEvent").unwrap();
        let poll_event = elysium_mem::next_abs_addr_mut(poll_event as *mut PollEvent);
        (*hooks_ref).poll_event = poll_event.replace(hooks::poll_event);

        hooked("SDL_PollEvent");

        state.hooks = Some(hooks.assume_init());

        println!("create gold");
        state.materials.gold = Some({
            use elysium_sdk::material::{Material, MaterialKind};
            use std::ptr;

            let state = State::get();
            let hooks = state.hooks.as_ref().unwrap_unchecked();
            let material = MaterialKind::Glow;
            let vdf =
                &*(hooks.vdf_from_bytes)(material.base_ptr(), material.vdf_ptr(), ptr::null());

            let material = &*material_system
                .create(material.name(), vdf)
                .cast::<Material>();

            println!("name = {:?}", material.name());
            println!("texture_group = {:?}", material.texture_group());

            material
        });
    }
}
