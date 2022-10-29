use std::cell::UnsafeCell;
use std::ptr;

/// A very unsafe structure used to pass objects by-reference through a static.
pub struct Scope<T> {
    cell: UnsafeCell<*const T>,
}

impl<T> Scope<T> {
    /// Construct an empty (null) scope.
    pub const fn new() -> Self {
        let cell = UnsafeCell::new(ptr::null());

        Self { cell }
    }

    /// Set the reference of the scope.
    ///
    /// # Safety
    ///
    /// This method offers no synchronization.
    pub unsafe fn set(&self, value: *const T) {
        self.cell.get().write(value);
    }

    /// Obtain the reference of the scope.
    ///
    /// # Safety
    ///
    /// This method offers no synchronization.
    /// The underlying reference may be null.
    pub unsafe fn get(&self) -> &T {
        &**self.cell.get()
    }
}

// `Scope` isn't actually `Sync`, but it's required if you want to put it in a static.
unsafe impl<T> Sync for Scope<T> {}
