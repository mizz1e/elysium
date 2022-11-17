use crate::{Button, CUserCmd};
use std::cmp::Ordering;

const MOVE: Button = Button::MOVE_FORWARD
    .union(Button::MOVE_BACKWARD)
    .union(Button::MOVE_LEFT)
    .union(Button::MOVE_RIGHT);

struct Buttons {
    forward: Button,
    backward: Button,
    left: Button,
    right: Button,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WalkingAnimation {
    /// Regular walking animation.
    #[default]
    Enabled,

    /// Slides across the ground.
    Disabled,
}

impl WalkingAnimation {
    #[inline]
    pub(crate) fn apply(&mut self, command: &mut CUserCmd) {
        let buttons = match self {
            WalkingAnimation::Enabled => Buttons {
                forward: Button::MOVE_FORWARD,
                backward: Button::MOVE_BACKWARD,
                left: Button::MOVE_LEFT,
                right: Button::MOVE_RIGHT,
            },
            WalkingAnimation::Disabled => Buttons {
                forward: Button::MOVE_BACKWARD,
                backward: Button::MOVE_FORWARD,
                left: Button::MOVE_RIGHT,
                right: Button::MOVE_LEFT,
            },
        };

        command.buttons.remove(MOVE);

        match command.movement.x.partial_cmp(&0.0_f32) {
            Some(Ordering::Greater) => command.buttons.insert(buttons.forward),
            Some(Ordering::Less) => command.buttons.insert(buttons.backward),
            _ => {}
        }

        match command.movement.y.partial_cmp(&0.0_f32) {
            Some(Ordering::Greater) => command.buttons.insert(buttons.right),
            Some(Ordering::Less) => command.buttons.insert(buttons.left),
            _ => {}
        }
    }
}
