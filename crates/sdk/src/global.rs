//! Globally accessible things.

use bevy::app::App;
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
#[inline]
pub unsafe fn with_app<T>(mut f: impl FnMut(&App) -> T) -> T {
    let app_ref = &*APP.0.get();
    let app = app_ref.as_ref().unwrap_unchecked();

    f(app)
}

/// Passes a mutable reference to the global [`App`] to the provided closure.
///
/// # Safety
///
/// The global [`App`] must be initialized prior.
#[inline]
pub unsafe fn with_app_mut<T>(mut f: impl FnMut(&mut App) -> T) -> T {
    let app_mut = &mut *APP.0.get();
    let app = app_mut.as_mut().unwrap_unchecked();

    f(app)
}

/// Set the global [`App`].
#[inline]
pub fn set_app(app: App) {
    unsafe {
        APP.0.get().write(Some(app));
    }
}
