use iced_native::{
    event::Event as IcedEvent,
    keyboard::{Event as IcedKeyEvent, KeyCode as IcedKeyCode, Modifiers as IcedKeyModifier},
    mouse::{Button as IcedMouseButton, Event as IcedMouseEvent},
    window::Event as IcedWindowEvent,
    Point,
};

use sdl2::{
    event::{Event as SdlEvent, WindowEvent as SdlWindowEvent},
    keyboard::{Keycode as SdlKeyCode, Mod as SdlKeyModifier},
    mouse::MouseButton as SdlMouseButton,
    sys::SDL_Event as RawSdlEvent,
};

pub fn map_event(raw: RawSdlEvent, mut emit: impl FnMut(IcedEvent)) {
    let event = SdlEvent::from_ll(raw);

    /*if !matches!(event, SdlEvent::Unknown { .. }) {
        tracing::debug!("{event:?}");
    }*/

    match event {
        SdlEvent::DropFile { filename, .. } => emit(IcedEvent::Window(
            IcedWindowEvent::FileDropped(filename.into()),
        )),

        SdlEvent::KeyDown {
            keycode, keymod, ..
        } => map_key(keycode, keymod, true, |event| {
            emit(IcedEvent::Keyboard(event))
        }),

        SdlEvent::KeyUp {
            keycode,
            keymod,
            repeat: false,
            ..
        } => map_key(keycode, keymod, false, |event| {
            emit(IcedEvent::Keyboard(event))
        }),

        SdlEvent::MouseButtonDown {
            which, mouse_btn, ..
        } => emit(IcedEvent::Mouse(map_mouse(mouse_btn, which, true))),

        SdlEvent::MouseButtonUp {
            which, mouse_btn, ..
        } => emit(IcedEvent::Mouse(map_mouse(mouse_btn, which, false))),

        SdlEvent::MouseMotion { x, y, .. } => emit(IcedEvent::Mouse(IcedMouseEvent::CursorMoved {
            position: Point::new(x as f32, y as f32),
        })),

        SdlEvent::TextInput { text, .. } => {
            for character in text.chars() {
                emit(IcedEvent::Keyboard(IcedKeyEvent::CharacterReceived(
                    character,
                )));
            }
        }

        SdlEvent::Window { win_event, .. } => {
            if let Some(event) = map_window(win_event) {
                emit(event);
            }
        }

        _ => {}
    }
}

fn map_mouse(button: SdlMouseButton, raw: u32, pressed: bool) -> IcedMouseEvent {
    let button = match button {
        SdlMouseButton::Left => IcedMouseButton::Left,
        SdlMouseButton::Middle => IcedMouseButton::Middle,
        SdlMouseButton::Right => IcedMouseButton::Right,
        SdlMouseButton::X1 => IcedMouseButton::Other(4),
        SdlMouseButton::X2 => IcedMouseButton::Other(5),
        _ => IcedMouseButton::Other(raw as u8),
    };

    if pressed {
        IcedMouseEvent::ButtonPressed(button)
    } else {
        IcedMouseEvent::ButtonReleased(button)
    }
}

fn map_window(event: SdlWindowEvent) -> Option<IcedEvent> {
    let event = match event {
        SdlWindowEvent::Enter => IcedEvent::Mouse(IcedMouseEvent::CursorEntered),
        SdlWindowEvent::FocusGained => IcedEvent::Window(IcedWindowEvent::Focused),
        SdlWindowEvent::FocusLost => IcedEvent::Window(IcedWindowEvent::Unfocused),
        SdlWindowEvent::Leave => IcedEvent::Mouse(IcedMouseEvent::CursorLeft),
        SdlWindowEvent::Moved(x, y) => IcedEvent::Window(IcedWindowEvent::Moved { x, y }),
        SdlWindowEvent::Resized(width, height) | SdlWindowEvent::SizeChanged(width, height) => {
            IcedEvent::Window(IcedWindowEvent::Resized {
                width: width as u32,
                height: height as u32,
            })
        }
        _ => return None,
    };

    Some(event)
}

fn map_key(
    sdl_key_code: Option<SdlKeyCode>,
    modifier: SdlKeyModifier,
    pressed: bool,
    mut emit: impl FnMut(IcedKeyEvent),
) {
    let modifiers = map_modifier(modifier);
    let event = if let Some(key_code) = sdl_key_code.and_then(map_key_code) {
        if pressed {
            IcedKeyEvent::KeyPressed {
                key_code,
                modifiers,
            }
        } else {
            IcedKeyEvent::KeyReleased {
                key_code,
                modifiers,
            }
        }
    } else {
        IcedKeyEvent::ModifiersChanged(modifiers)
    };

    emit(event);
}

fn map_key_code(key_code: SdlKeyCode) -> Option<IcedKeyCode> {
    let key_code = match key_code {
        SdlKeyCode::A => IcedKeyCode::A,
        SdlKeyCode::B => IcedKeyCode::B,
        SdlKeyCode::C => IcedKeyCode::C,
        SdlKeyCode::D => IcedKeyCode::D,
        SdlKeyCode::E => IcedKeyCode::E,
        SdlKeyCode::F => IcedKeyCode::F,
        SdlKeyCode::G => IcedKeyCode::G,
        SdlKeyCode::H => IcedKeyCode::H,
        SdlKeyCode::I => IcedKeyCode::I,
        SdlKeyCode::J => IcedKeyCode::J,
        SdlKeyCode::K => IcedKeyCode::K,
        SdlKeyCode::L => IcedKeyCode::L,
        SdlKeyCode::M => IcedKeyCode::M,
        SdlKeyCode::N => IcedKeyCode::N,
        SdlKeyCode::O => IcedKeyCode::O,
        SdlKeyCode::P => IcedKeyCode::P,
        SdlKeyCode::Q => IcedKeyCode::Q,
        SdlKeyCode::R => IcedKeyCode::R,
        SdlKeyCode::S => IcedKeyCode::S,
        SdlKeyCode::T => IcedKeyCode::T,
        SdlKeyCode::U => IcedKeyCode::U,
        SdlKeyCode::V => IcedKeyCode::V,
        SdlKeyCode::W => IcedKeyCode::W,
        SdlKeyCode::X => IcedKeyCode::X,
        SdlKeyCode::Y => IcedKeyCode::Y,
        SdlKeyCode::Z => IcedKeyCode::Z,

        SdlKeyCode::Num0 => IcedKeyCode::Key0,
        SdlKeyCode::Num1 => IcedKeyCode::Key1,
        SdlKeyCode::Num2 => IcedKeyCode::Key2,
        SdlKeyCode::Num3 => IcedKeyCode::Key3,
        SdlKeyCode::Num4 => IcedKeyCode::Key4,
        SdlKeyCode::Num5 => IcedKeyCode::Key5,
        SdlKeyCode::Num6 => IcedKeyCode::Key6,
        SdlKeyCode::Num7 => IcedKeyCode::Key7,
        SdlKeyCode::Num8 => IcedKeyCode::Key8,
        SdlKeyCode::Num9 => IcedKeyCode::Key9,

        SdlKeyCode::Asterisk => IcedKeyCode::Asterisk,
        SdlKeyCode::Backspace => IcedKeyCode::Backspace,
        SdlKeyCode::Comma => IcedKeyCode::Comma,
        SdlKeyCode::Delete => IcedKeyCode::Delete,
        SdlKeyCode::Down => IcedKeyCode::Down,
        SdlKeyCode::End => IcedKeyCode::End,
        SdlKeyCode::Escape => IcedKeyCode::Escape,
        SdlKeyCode::Home => IcedKeyCode::Home,
        SdlKeyCode::Insert => IcedKeyCode::Insert,
        SdlKeyCode::Left => IcedKeyCode::Left,
        SdlKeyCode::PageDown => IcedKeyCode::PageDown,
        SdlKeyCode::PageUp => IcedKeyCode::PageUp,
        SdlKeyCode::Period => IcedKeyCode::Period,
        SdlKeyCode::Plus => IcedKeyCode::Plus,
        SdlKeyCode::Return | SdlKeyCode::Return2 => IcedKeyCode::Enter,
        SdlKeyCode::Right => IcedKeyCode::Right,
        SdlKeyCode::Space => IcedKeyCode::Space,
        SdlKeyCode::Tab => IcedKeyCode::Tab,
        SdlKeyCode::Underscore => IcedKeyCode::Underline,
        SdlKeyCode::Up => IcedKeyCode::Up,

        _ => return None,
    };

    Some(key_code)
}

fn map_modifier(modifier: SdlKeyModifier) -> IcedKeyModifier {
    let mut new_modifier = IcedKeyModifier::empty();

    if modifier.contains(SdlKeyModifier::LALTMOD | SdlKeyModifier::RALTMOD) {
        new_modifier.insert(IcedKeyModifier::ALT);
    }

    if modifier.contains(SdlKeyModifier::LCTRLMOD | SdlKeyModifier::RCTRLMOD) {
        new_modifier.insert(IcedKeyModifier::CTRL);
    }

    if modifier.contains(SdlKeyModifier::LSHIFTMOD | SdlKeyModifier::RSHIFTMOD) {
        new_modifier.insert(IcedKeyModifier::SHIFT);
    }

    new_modifier
}
