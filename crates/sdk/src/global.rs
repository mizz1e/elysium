//! Globally accessible things.

use bevy::app::App;
use bevy::ecs::prelude::*;
use std::cell::UnsafeCell;

static APP: AppGlobal = AppGlobal(UnsafeCell::new(None));

/// Wrapper of [`App`] to implement [`Send`].
struct AppGlobal(UnsafeCell<Option<App>>);

unsafe impl Send for AppGlobal {}
unsafe impl Sync for AppGlobal {}

/// Passes a reference to the global [`App`] to the provided closure.
///
/// # Safety
///
/// The global [`App`] must be initialized prior.
pub fn with_app<T>(f: impl FnOnce(&App) -> T) -> T {
    let app_ref = unsafe { &*APP.0.get() };
    let app = app_ref.as_ref().unwrap();

    f(app)
}

/// Passes a mutable reference to the global [`App`] to the provided closure.
///
/// # Safety
///
/// The global [`App`] must be initialized prior.
pub fn with_app_mut<T>(f: impl FnOnce(&mut App) -> T) -> T {
    let app_mut = unsafe { &mut *APP.0.get() };
    let app = app_mut.as_mut().unwrap();

    f(app)
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

/// Set the global [`App`].
pub fn set_app(app: App) {
    unsafe {
        APP.0.get().write(Some(app));
    }
}
