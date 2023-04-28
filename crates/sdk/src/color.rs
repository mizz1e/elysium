use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub fn from_rgba_u32(rgba: u32) -> Self {
        let [red, green, blue, alpha] = rgba.to_le_bytes();

        Color {
            red: (red as f32) / 255.0,
            green: (green as f32) / 255.0,
            blue: (blue as f32) / 255.0,
            alpha: (alpha as f32) / 255.0,
        }
    }

    pub fn to_rgba_u32(self) -> u32 {
        let Self {
            red,
            green,
            blue,
            alpha,
        } = self;

        u32::from_le_bytes([
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
            (alpha * 255.0) as u8,
        ])
    }

    pub fn from_hex_str(string: &str) -> Self {
        u32::from_str_radix(string, 16)
            .map(Self::from_rgba_u32)
            .unwrap_or_default()
    }

    pub fn to_hex_string(self) -> String {
        let rgba = self.to_rgba_u32();

        format!("{rgba:08X}")
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.red.to_ne_bytes().hash(state);
        self.green.to_ne_bytes().hash(state);
        self.blue.to_ne_bytes().hash(state);
        self.alpha.to_ne_bytes().hash(state);
    }
}
