use {bevy::prelude::*, std::cell::UnsafeCell};

static CELL: Cell = Cell(UnsafeCell::new(None));

struct Cell(UnsafeCell<Option<App>>);

unsafe impl Send for Cell {}
unsafe impl Sync for Cell {}

/// # Safety
///
/// [`App`](App) manages most of it's safety.
pub unsafe fn set(app: App) {
    let cell = unsafe { &mut *CELL.0.get() };

    if cell.is_some() {
        panic!("App already set");
    }

    *cell = Some(app);
}

/// # Safety
///
/// [`App`](App) manages most of it's safety.
pub unsafe fn get() -> &'static App {
    let cell = unsafe { &*CELL.0.get() };

    cell.as_ref().unwrap_or_else(|| {
        panic!("App was not set");
    })
}

/// # Safety
///
/// [`App`](App) manages most of it's safety.
pub unsafe fn get_mut() -> &'static mut App {
    let cell = unsafe { &mut *CELL.0.get() };

    cell.as_mut().unwrap_or_else(|| {
        panic!("App was not set");
    })
}
