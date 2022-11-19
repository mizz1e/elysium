use crate::{global, iced, Config};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use iced_glow::Viewport;
use iced_native::keyboard::Event::KeyPressed;
use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button::Other;
use iced_native::mouse::Event::ButtonPressed;
use iced_native::{mouse, Event, Point, Size};
use sdl2_sys::{SDL_Event, SDL_GL_SwapWindow, SDL_GetWindowSize, SDL_PollEvent, SDL_Window};
use std::{ffi, ptr};

#[derive(Resource)]
pub struct PollEvent(pub unsafe extern "C" fn(event: *mut SDL_Event) -> ffi::c_int);

#[derive(Resource)]
pub struct SwapWindow(pub unsafe extern "C" fn(event: *mut SDL_Window));

#[derive(Resource)]
pub struct WindowViewport(pub Viewport);

#[derive(Resource)]
pub struct CursorPosition(pub Point);

pub unsafe fn setup() -> (PollEvent, SwapWindow) {
    let method = unsafe { elysium_mem::next_abs_addr_mut_ptr(SDL_PollEvent as *mut u8).unwrap() };

    let poll_event = PollEvent(ptr::replace(method, poll_event));

    let method =
        unsafe { elysium_mem::next_abs_addr_mut_ptr(SDL_GL_SwapWindow as *mut u8).unwrap() };

    let swap_window = SwapWindow(ptr::replace(method, swap_window));

    (poll_event, swap_window)
}

unsafe extern "C" fn poll_event(event: *mut SDL_Event) -> ffi::c_int {
    let method = global::with_app(|app| app.world.resource::<PollEvent>().0);
    let result = (method)(event);

    global::with_app_mut(|app| {
        elysium_input::map_event(*event, |event| {
            let mut system_state: SystemState<(
                ResMut<Config>,
                ResMut<CursorPosition>,
                ResMut<iced::IcedProgram<iced::Menu>>,
            )> = SystemState::new(&mut app.world);

            let (mut config, mut cursor_position, mut program) =
                system_state.get_mut(&mut app.world);

            match &event {
                // insert
                Event::Keyboard(KeyPressed {
                    key_code: KeyCode::Insert,
                    ..
                }) => config.menu_open ^= true,

                // insert
                Event::Keyboard(KeyPressed {
                    key_code: KeyCode::Escape,
                    ..
                }) => config.menu_open = false,

                // thirdperson
                Event::Mouse(ButtonPressed(Other(4))) => config.thirdperson_enabled ^= true,

                // move cursor
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    cursor_position.0 = *position
                }
                _ => {}
            };

            program.queue_event(event);
        });
    });

    result
}

unsafe extern "C" fn swap_window(window: *mut SDL_Window) {
    let method = global::with_app(|app| app.world.resource::<SwapWindow>().0);
    let (mut width, mut height) = (0, 0);

    SDL_GetWindowSize(window, &mut width, &mut height);

    global::with_app_mut(move |app| {
        let viewport = Viewport::with_physical_size(
            Size {
                width: width as u32,
                height: height as u32,
            },
            1.0,
        );

        app.insert_resource(WindowViewport(viewport));

        iced::render();
    });

    (method)(window)
}
