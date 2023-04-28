use {
    bevy::prelude::*,
    bevy_source_internal::{app, app_mut},
};

/// Passes a reference to the global [`App`] to the provided closure.
///
/// # Safety
///
/// The global [`App`] must be initialized prior.
pub fn with_app<T>(f: impl FnOnce(&App) -> T) -> T {
    f(app!())
}

/// Passes a mutable reference to the global [`App`] to the provided closure.
///
/// # Safety
///
/// The global [`App`] must be initialized prior.
pub fn with_app_mut<T>(f: impl FnOnce(&mut App) -> T) -> T {
    f(app_mut!())
}

pub fn with_resource<R: Resource, T>(f: impl FnOnce(&R) -> T) -> T {
    with_app(|app| f(app.world.resource()))
}

pub fn with_resource_mut<R: Resource, T>(f: impl FnOnce(Mut<'_, R>) -> T) -> T {
    with_app_mut(|app| f(app.world.resource_mut()))
}

pub fn with_resource_or_init<R: Resource, T>(
    f: impl FnOnce(Mut<'_, R>) -> T,
    init: impl FnOnce() -> R,
) -> T {
    with_app_mut(|app| f(app.world.get_resource_or_insert_with(init)))
}
