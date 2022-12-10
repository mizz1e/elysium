use crate::{global, iced, Config};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use iced_glow::Viewport;
use iced_native::keyboard::Event::{KeyPressed, KeyReleased};
use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button::Other;
use iced_native::mouse::Event::ButtonPressed;
use iced_native::{mouse, window, Event, Point, Size};
use sdl2::sys;
use std::collections::HashSet;
use std::{ffi, ptr};

pub mod conversion;

#[derive(Resource)]
pub struct PollEvent(pub unsafe extern "C" fn(event: *mut sys::SDL_Event) -> ffi::c_int);

#[derive(Resource)]
pub struct SwapWindow(pub unsafe extern "C" fn(event: *mut sys::SDL_Window));

#[derive(Resource)]
pub struct WindowViewport(pub Viewport);

#[derive(Resource)]
pub struct CursorPosition(pub Point);

pub unsafe fn setup() -> (PollEvent, SwapWindow) {
    let method =
        unsafe { elysium_mem::next_abs_addr_mut_ptr(sys::SDL_PollEvent as *mut u8).unwrap() };

    let poll_event = PollEvent(ptr::replace(method, poll_event));

    let method =
        unsafe { elysium_mem::next_abs_addr_mut_ptr(sys::SDL_GL_SwapWindow as *mut u8).unwrap() };

    let swap_window = SwapWindow(ptr::replace(method, swap_window));

    (poll_event, swap_window)
}

unsafe extern "C" fn poll_event(event: *mut sys::SDL_Event) -> ffi::c_int {
    let method = global::with_app(|app| app.world.resource::<PollEvent>().0);
    let result = (method)(event);

    global::with_app_mut(|app| {
        conversion::map_event(*event, |event| {
            let mut system_state: SystemState<(
                ResMut<Config>,
                ResMut<CursorPosition>,
                ResMut<iced::IcedProgram<iced::Menu>>,
            )> = SystemState::new(&mut app.world);

            let (mut config, mut cursor_position, mut menu) = system_state.get_mut(&mut app.world);

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

            if config.menu_open {
                menu.queue_event(event);
            }
        });
    });

    result
}

unsafe extern "C" fn swap_window(window: *mut sys::SDL_Window) {
    let method = global::with_app(|app| app.world.resource::<SwapWindow>().0);
    let (mut width, mut height) = (0, 0);

    sys::SDL_GetWindowSize(window, &mut width, &mut height);

    let width = width as u32;
    let height = height as u32;

    global::with_app_mut(move |app| {
        let new_viewport = Viewport::with_physical_size(Size { width, height }, 1.0);

        if let Some(viewport) = app.world.get_resource::<WindowViewport>() {
            if viewport.0.physical_size() != new_viewport.physical_size() {
                let mut system_state: SystemState<(
                    ResMut<iced::IcedProgram<iced::Hud>>,
                    ResMut<iced::IcedProgram<iced::Menu>>,
                )> = SystemState::new(&mut app.world);

                let (mut hud, mut menu) = system_state.get_mut(&mut app.world);
                let event = Event::Window(window::Event::Resized { width, height });

                hud.queue_event(event.clone());
                menu.queue_event(event);
            }
        }

        app.insert_resource(WindowViewport(new_viewport));

        iced::render();
    });

    (method)(window)
}
