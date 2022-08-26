use super::Entity;
use palette::Srgba;
use std::ops::RangeInclusive;

/// Fog methods.
pub trait Fog<'a>: Entity<'a> {
    /// Returns the clip distance (far-Z).
    ///
    /// Distance is relative to the local players position.
    fn clip_distance(&self) -> f32;

    /// Returns the distance range (start and end distance).
    ///
    /// Distance is relative to the local players position.
    fn range(&self) -> Option<RangeInclusive<f32>>;

    /// Returns the color.
    fn rgba(&self) -> Srgba;

    /// Set the clip distance (far-Z).
    ///
    /// A non-finite, negative or zero value will disable the clip distance.
    ///
    /// Distance is relative to the local players position.
    fn set_clip_distance(&mut self, distance: f32);

    /// Set the distance range (start and end distance).
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    ///
    /// Distance is relative to the local players position.
    fn set_range(&mut self, distance: Option<RangeInclusive<f32>>);

    /// Set the color.
    fn set_rgba(&mut self, srgba: impl Into<Srgba>);
}
