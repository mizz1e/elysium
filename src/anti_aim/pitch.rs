use core::fmt;

/// `65536`.
pub const MAX_I32: i32 = 65536;

/// `65536.0`.
pub const MAX_F32: f32 = MAX_I32 as f32;

/// `360.0 / 65536.0`.
pub const ANGLE_MAX: f32 = 360.0 / MAX_F32;

/// `65536.0 / 360.0`.
pub const MAX_ANGLE: f32 = MAX_F32 / 360.0;

// NOTE: this was patched
// tl;dr angle_mod couldn't handle values above this properly
// https://www.unknowncheats.me/forum/1747547-post9.html
pub const LISP: f32 = MAX_F32 * 360.0;

// https://github.com/id-Software/Quake/blob/master/QW/client/mathlib.c#L154
#[inline]
pub const fn old_angle_mod(value: f32) -> f32 {
    ANGLE_MAX * (((value * MAX_ANGLE) as i32) & MAX_I32) as f32
}

/// `89.0`.
///
/// Pitch down.
pub const MAX_PITCH: f32 = 89.0;

/// `-89.0`.
///
/// Pitch up.
pub const MIN_PITCH: f32 = -89.0;

/// `3.0`.
pub const FAKE_ROTATIONS: f32 = 3.0;

/// `360.0 * 3.0`.
///
/// This angle after normalization and clamping is 0, but the server still processes the original
/// angle.
pub const FAKE_DOWN: f32 = 360.0 * FAKE_ROTATIONS;

/// `-360.0 * 3.0`.
///
/// This angle after normalization and clamping is 0, but the server still processes the original
/// angle.
pub const FAKE_UP: f32 = -360.0 * FAKE_ROTATIONS;

/// Pitch modifier variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pitch {
    /// Default behaviour.
    Default,

    /// The player looks up.
    Up,

    /// The player looks directly forward.
    Zero,

    /// The player looks down.
    Down,

    /// Clients receive a sanitized angle (`0.0`), server processes angle as up.
    ///
    /// This will cause untrusted.
    FakeUp,

    /// Clients receive a sanitized angle (`0.0`), server processes angle as down.
    ///
    /// This will cause untrusted.
    FakeDown,

    /// Causes overflow in the older [`anglemod`](https://github.com/id-Software/Quake/blob/master/QW/client/mathlib.c#L154) implementation.
    ///
    /// Patched in current CS:GO.
    ///
    /// This will cause untrusted.
    Lisp,
}

impl Pitch {
    /// Default value for pitch.
    #[inline]
    pub const fn new() -> Self {
        Self::Default
    }

    /// Obtain the pitch angle.
    #[inline]
    pub const fn value(self) -> Option<f32> {
        let value = match self {
            Pitch::Up => MIN_PITCH,
            Pitch::Zero => 0.0,
            Pitch::Down => MAX_PITCH,
            Pitch::FakeUp => FAKE_UP,
            Pitch::FakeDown => FAKE_DOWN,
            Pitch::Lisp => LISP,
            // TODO: Research these values (Can't seem to find anything!)
            // AFAIK it's just like lisp.
            //Pitch::AngelUp => 35999912.0,
            //Pitch::AngelDown => 36000088.0,
            _ => return None,
        };

        Some(value)
    }

    /// Determine whether this pitch will cause untrusted.
    #[inline]
    pub const fn is_untrusted(self) -> bool {
        !matches!(self, Pitch::Default | Pitch::Up | Pitch::Zero | Pitch::Down)
    }

    /// Apply the pitch modifier to the provided angle.
    #[inline]
    pub fn apply(self, pitch: f32) -> f32 {
        self.value().unwrap_or(pitch)
    }

    /// Returns a string for the UI.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Pitch::Default => "Default",
            Pitch::Up => "Up",
            Pitch::Zero => "Zero",
            Pitch::Down => "Down",
            Pitch::FakeUp => "Fake Up",
            Pitch::FakeDown => "Fake Down",
            Pitch::Lisp => "Lisp",
        }
    }
}

impl fmt::Display for Pitch {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}
