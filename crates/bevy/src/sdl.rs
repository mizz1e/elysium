use {
    bevy::{
        ecs::system::SystemParam,
        input::{
            keyboard::KeyboardInput,
            mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
            touch::TouchInput,
            ButtonState,
        },
        prelude::*,
        window::{
            WindowBackendScaleFactorChanged, WindowCloseRequested, WindowFocused, WindowResized,
            WindowScaleFactorChanged,
        },
    },
    sdl2::event,
};

pub mod convert;
pub mod gl_swap_window;
pub mod poll_event;

/// Source Engine SDL integration.
#[derive(Default)]
pub struct SdlPlugin;

#[derive(Resource)]
pub struct SdlContext {
    pub gl_swap_window: gl_swap_window::Fn,
    pub poll_event: poll_event::Fn,
    pub is_initialized: bool,
}

impl Plugin for SdlPlugin {
    fn build(&self, app: &mut App) {
        let poll_event = poll_event::add_schedule(app, poll_event).unwrap();
        let gl_swap_window = gl_swap_window::add_schedule(app, gl_swap_window).unwrap();

        app.insert_resource(SdlContext {
            poll_event,
            gl_swap_window,
            is_initialized: false,
        });
    }
}

// Based on https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs
#[derive(SystemParam)]
pub struct WindowEvents<'w> {
    window_resized: EventWriter<'w, WindowResized>,
    window_close_requested: EventWriter<'w, WindowCloseRequested>,
    window_scale_factor_changed: EventWriter<'w, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventWriter<'w, WindowBackendScaleFactorChanged>,
    window_focused: EventWriter<'w, WindowFocused>,
    window_moved: EventWriter<'w, WindowMoved>,
}

#[derive(SystemParam)]
pub struct InputEvents<'w> {
    keyboard_input: EventWriter<'w, KeyboardInput>,
    character_input: EventWriter<'w, ReceivedCharacter>,
    mouse_button_input: EventWriter<'w, MouseButtonInput>,
    mouse_wheel_input: EventWriter<'w, MouseWheel>,
    touch_input: EventWriter<'w, TouchInput>,
    ime_input: EventWriter<'w, Ime>,
}

#[derive(SystemParam)]
pub struct CursorEvents<'w> {
    cursor_moved: EventWriter<'w, CursorMoved>,
    cursor_entered: EventWriter<'w, CursorEntered>,
    cursor_left: EventWriter<'w, CursorLeft>,
}

pub fn poll_event(
    In(event): In<event::Event>,
    window_entity: Query<Entity, With<Window>>,
    mut window_events: WindowEvents,
    mut input_events: InputEvents,
    mut cursor_events: CursorEvents,
) {
    use event::Event;

    let Ok(window_entity) = window_entity.get_single() else {
        return;
    };

    match event {
        Event::MouseButtonDown { mouse_btn, .. } => {
            input_events.mouse_button_input.send(MouseButtonInput {
                button: convert::mouse_button(mouse_btn),
                state: ButtonState::Pressed,
            });
        }
        Event::MouseButtonUp { mouse_btn, .. } => {
            input_events.mouse_button_input.send(MouseButtonInput {
                button: convert::mouse_button(mouse_btn),
                state: ButtonState::Released,
            });
        }
        Event::MouseMotion { x, y, .. } => {
            cursor_events.cursor_moved.send(CursorMoved {
                window: window_entity,
                position: Vec2::new(x as f32, y as f32),
            });
        }
        Event::MouseWheel { x, y, .. } => {
            input_events.mouse_wheel_input.send(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: x as f32,
                y: y as f32,
            });
        }
        Event::KeyDown {
            keycode,
            scancode: Some(scancode),
            repeat: false,
            ..
        } => {
            input_events.keyboard_input.send(KeyboardInput {
                scan_code: scancode as i32 as u32,
                key_code: keycode.and_then(convert::key_code),
                state: ButtonState::Pressed,
            });
        }
        Event::KeyUp {
            keycode,
            scancode: Some(scancode),
            repeat: false,
            ..
        } => {
            input_events.keyboard_input.send(KeyboardInput {
                scan_code: scancode as i32 as u32,
                key_code: keycode.and_then(convert::key_code),
                state: ButtonState::Released,
            });
        }
        _ => {}
    }
}

pub fn gl_swap_window() {
    //println!("swap window");
}
