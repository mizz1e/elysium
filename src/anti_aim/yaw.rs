use core::fmt;

/// Yaw modifier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Yaw {
    /// Base angle to apply (add) to yaw (`-180.0..=180.0`).
    pub base: f32,

    /// Modifier to apply (add) to yaw after base has been applied.
    pub modifier: YawModifier,
}

/// Yaw modifier variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YawModifier {
    /// Default behaviour.
    Default,

    /// Legit desync (`90.0`).
    Legit,

    /// Jitter (`min..=max`).
    Jitter(f32, f32),
}

/// Yaw modifier variant that implements Eq and ToString for an iced PickList.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum YawModifierKind {
    /// Default behaviour.
    Default,

    /// Legit desync.
    Legit,

    /// Jitter.
    Jitter,
}

impl Yaw {
    /// Default value for yaw.
    #[inline]
    pub const fn new() -> Self {
        let base = 0.0;
        let modifier = YawModifier::Default;

        Self { base, modifier }
    }

    /// Apply the yaw modifier to the provided angle.
    #[inline]
    pub fn apply(self, yaw: f32) -> f32 {
        yaw + self.base + self.modifier.value().unwrap_or(0.0)
    }
}

impl YawModifier {
    /// Obtain the yaw angle.
    #[inline]
    pub fn value(self) -> Option<f32> {
        let value = match self {
            YawModifier::Legit => 90.0,
            YawModifier::Jitter(min, max) => super::random(min..=max),
            _ => return None,
        };

        Some(value)
    }

    #[inline]
    pub fn kind(self) -> YawModifierKind {
        match self {
            YawModifier::Default => YawModifierKind::Default,
            YawModifier::Legit => YawModifierKind::Legit,
            YawModifier::Jitter(_min, _max) => YawModifierKind::Jitter,
        }
    }
}

impl YawModifierKind {
    /// Returns a string for the UI.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            YawModifierKind::Default => "Default",
            YawModifierKind::Legit => "Legit",
            YawModifierKind::Jitter => "Jitter",
        }
    }
}

impl fmt::Display for YawModifierKind {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}
