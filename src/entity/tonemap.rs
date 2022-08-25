use super::{Entity, Exposure};

/// Tonemap methods.
pub trait Tonemap<'a>: Entity<'a> {
    /// Returns the bloom effect scale.
    fn bloom(&self) -> f32;

    /// Returns the exposure range.
    fn exposure(&self) -> Option<Exposure>;

    /// Set the bloom effect scale.
    ///
    /// Non-finite or negative scale will be treated as 0.0.
    fn set_bloom(&mut self, scale: f32);

    /// Set the exposure range.
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    fn set_exposure<E: Into<Exposure>>(&mut self, exposure: Option<E>);
}
