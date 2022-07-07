//#![deny(warnings)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
// src/entity.rs
#![feature(const_ptr_offset_from)]
#![feature(abi_thiscall)]

use elysium_sdk::convar::Vars;
use elysium_sdk::model::ModelRender;
use elysium_sdk::{Client, Console, LibraryKind};
use std::path::Path;
use std::{mem, thread};

pub use elysium_state as state;

pub use entity::Entity;
pub use networked::Networked;

mod entity;

pub mod hooks;
pub mod library;
pub mod networked;
pub mod pattern;

// this is called by glibc after the library is loaded into a process
#[link_section = ".init_array"]
#[used]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

#[link_section = ".text.startup"]
unsafe extern "C" fn bootstrap() {
    // check the name of the process we're injected into
    let is_csgo = std::env::args()
        .next()
        .and_then(|process_path| {
            let process_path = Path::new(&process_path);
            let process_name = process_path.file_name()?;

            if process_name == "csgo_linux64" ||
                // https://github.com/elysian6969/csgo-launcher xoxo
                process_name == "csgo-launcher"
            {
                Some(true)
            } else {
                None
            }
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
fn main() {
    library::wait_for_serverbrowser();

    let interfaces = library::load_interfaces();

    unsafe {
        state::set_interfaces(mem::transmute_copy(&interfaces));
    }

    let console: &'static Console = unsafe { &*interfaces.convar.cast() };
    let client: &'static Client = unsafe { &*interfaces.client.cast() };
    let model_render: &'static ModelRender = unsafe { &*interfaces.model_render.cast() };
    let globals = client.globals();
    let input = client.input();

    console.write("welcome to elysium\n");

    let vars = Vars::from_loader(|var_kind| {
        let var_nul_name = var_kind.as_nul_str();
        let var_name = var_kind.as_str();
        let address = console.var(var_nul_name);

        println!("elysium | config variable \x1b[38;5;2m{var_name}\x1b[m found at \x1b[38;5;3m{address:?}\x1b[m");

        address
    });

    let networked = Networked::new(client);

    let gl = elysium_gl::Gl::open().expect("libGL");

    println!(
        "elysium | loaded \x1b[38;5;2mlibGL\x1b[m at \x1b[38;5;3m{:?}\x1b[m",
        gl
    );

    let sdl = elysium_sdl::Sdl::open().expect("libSDL");

    /*println!(
        "elysium | loaded \x1b[38;5;2mlibSDL\x1b[m at \x1b[38;5;3m{:?}\x1b[m",
        sdl
    );*/

    let swap_window = unsafe { sdl.swap_window() };
    let poll_event = unsafe { sdl.poll_event() };

    let _animation_layers = unsafe {
        let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_LAYERS).unwrap();

        bytes.as_ptr().byte_add(35).cast::<u32>().read()
    };

    let _animation_state = unsafe {
        let bytes = pattern::get(LibraryKind::Client, &pattern::ANIMATION_STATE).unwrap();

        bytes.as_ptr().byte_add(52).cast::<u32>().read()
    };

    unsafe {
        let bytes = pattern::get(LibraryKind::Engine, &pattern::CL_MOVE).unwrap();
        // convert to function pointer
        let fun = mem::transmute(bytes.as_ptr());

        state::hooks::set_cl_move(fun);
    }

    unsafe {
        let bytes = pattern::get(LibraryKind::Client, &pattern::VDF_INIT).unwrap();
        // convert to function pointer
        let fun = mem::transmute(bytes.as_ptr());

        state::hooks::set_vdf_init(fun);
    }

    unsafe {
        let bytes = pattern::get(LibraryKind::Client, &pattern::VDF_FROM_BYTES).unwrap();
        // convert to function pointer
        let fun = mem::transmute(bytes.as_ptr());

        state::hooks::set_vdf_from_bytes(fun);
    }

    unsafe {
        let gl_context = elysium_gl::Context::new(|symbol| gl.get_proc_address(symbol).cast());

        state::set_gl(gl);
        state::set_sdl(sdl);

        state::set_gl_context(gl_context);

        state::set_networked(mem::transmute(networked));
        state::set_vars(mem::transmute(vars));

        state::set_engine(interfaces.engine);
        state::set_input_system(interfaces.input_system);
        state::set_entity_list(interfaces.entity_list);
        state::set_globals(globals);
        state::set_input(input);

        {
            let address = client
                .create_move_address()
                .as_mut()
                .cast::<state::hooks::CreateMove>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_create_move(address.replace(hooks::create_move));
            println!("elysium | hooked \x1b[38;5;2mCreateMove\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        {
            let address = model_render
                .draw_model_address()
                .as_mut()
                .cast::<state::hooks::DrawModel>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_draw_model(address.replace(hooks::draw_model));
            println!("elysium | hooked \x1b[38;5;2mDrawModelExecute\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        {
            let address = client
                .frame_stage_notify_address()
                .as_mut()
                .cast::<state::hooks::FrameStageNotify>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_frame_stage_notify(address.replace(hooks::frame_stage_notify));
            println!("elysium | hooked \x1b[38;5;2mFrameStageNotify\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        {
            let address = client
                .override_view_address()
                .as_mut()
                .cast::<state::hooks::OverrideView>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_override_view(address.replace(hooks::override_view));
            println!("elysium | hooked \x1b[38;5;2mOverrideView\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        state::hooks::set_swap_window(
            swap_window
                .as_mut()
                .cast::<state::hooks::SwapWindow>()
                .replace(hooks::SWAP_WINDOW),
        );

        println!("elysium | hooked \x1b[38;5;2mSDL_GL_SwapWindow\x1b[m");

        state::hooks::set_poll_event(
            poll_event
                .as_mut()
                .cast::<state::hooks::PollEvent>()
                .replace(hooks::POLL_EVENT),
        );

        println!("elysium | hooked \x1b[38;5;2mSDL_PollEvent\x1b[m");
    }
}
