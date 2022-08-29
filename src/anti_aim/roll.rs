use core::fmt;

/// Roll modifier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Roll {
    /// Base angle to apply (add) to base (`-50.0..=50.0`).
    pub base: f32,

    /// Modifier to apply (add) to roll after base has been applied.
    pub modifier: RollModifier,
}

/// Roll modifier variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RollModifier {
    /// Default behaviour.
    Default,

    /// Jitter (`min..=max`).
    Jitter(f32, f32),
}

/// Roll modifier variant that implements Eq and ToString for an iced PickList.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RollModifierKind {
    /// Default behaviour.
    Default,

    /// Jitter.
    Jitter,
}

impl Roll {
    /// Default value for roll.
    #[inline]
    pub const fn new() -> Self {
        let base = 0.0;
        let modifier = RollModifier::Default;

        Self { base, modifier }
    }

    /// Apply the roll modifier to the provided angle.
    #[inline]
    pub fn apply(self, roll: f32) -> f32 {
        roll + self.base + self.modifier.value().unwrap_or(0.0)
    }
}

impl RollModifier {
    /// Obtain the roll angle.
    #[inline]
    pub fn value(self) -> Option<f32> {
        let value = match self {
            RollModifier::Jitter(min, max) => super::random(min..=max),
            _ => return None,
        };

        Some(value)
    }

    #[inline]
    pub fn kind(self) -> RollModifierKind {
        match self {
            RollModifier::Default => RollModifierKind::Default,
            RollModifier::Jitter(_min, _max) => RollModifierKind::Jitter,
        }
    }
}

impl RollModifierKind {
    /// Returns a string for the UI.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            RollModifierKind::Default => "Default",
            RollModifierKind::Jitter => "Jitter",
        }
    }
}

impl fmt::Display for RollModifierKind {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}
