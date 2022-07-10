use crate::State;
use iced_native::keyboard::Event::{KeyPressed, KeyReleased};
use iced_native::keyboard::KeyCode::Insert;
use iced_native::mouse::Button::Other;
use iced_native::mouse::Event::{ButtonPressed, ButtonReleased};
use iced_native::{mouse, Event};

/// `SDL_PollEvent` hook.
pub unsafe extern "C" fn poll_event(event: *mut sdl2_sys::SDL_Event) -> i32 {
    frosting::println!();

    let state = State::get();
    let local_vars = &mut state.local;
    let hooks = state.hooks.as_ref().unwrap_unchecked();
    let result = (hooks.poll_event)(event);
    let menu = state.menu.as_mut();

    if let Some(menu) = menu {
        elysium_input::map_event(*event, |event| {
            let state = State::get();

            match &event {
                // insert
                Event::Keyboard(KeyPressed {
                    key_code: Insert, ..
                }) => state.toggle_menu(),

                // thirdperson
                Event::Mouse(ButtonPressed(Other(4))) => local_vars.toggle_thirdperson(),

                // p100 duplicate input fixes
                // insert
                Event::Keyboard(KeyReleased {
                    key_code: Insert, ..
                }) => state.release_menu_toggle(),

                // thirdperson
                Event::Mouse(ButtonReleased(Other(4))) => local_vars.release_thirdperson_toggle(),

                // move cursor
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state.cursor_position = *position;
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    // block input to the game when the menu is open
    if state.menu_open.0 {
        (*event).type_ = 0;
    }

    result
}
