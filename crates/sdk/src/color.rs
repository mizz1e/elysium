use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.red.to_ne_bytes().hash(state);
        self.green.to_ne_bytes().hash(state);
        self.blue.to_ne_bytes().hash(state);
        self.alpha.to_ne_bytes().hash(state);
    }
}

impl From<iced_native::Color> for Color {
    fn from(color: iced_native::Color) -> Color {
        let iced_native::Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        } = color;

        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Into<iced_native::Color> for Color {
    fn into(self) -> iced_native::Color {
        let Color {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        } = self;

        iced_native::Color { r, g, b, a }
    }
}
