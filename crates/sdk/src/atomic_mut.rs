use core::marker::PhantomData;
use core::ptr::Thin;
use core::{mem, ptr};
use crossbeam_utils::atomic::AtomicCell;

/// A thread-safe `Option<&mut T>`.
#[repr(transparent)]
pub struct AtomicMut<'a, T: Thin + ?Sized> {
    cell: AtomicCell<*mut T>,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Thin + ?Sized> AtomicMut<'a, T> {
    /// Create a new mutable, optional, atomic reference.
    ///
    /// Initailized to `None`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            cell: AtomicCell::new(ptr::null_mut()),
            _phantom: PhantomData,
        }
    }

    /// Loads the mutable reference.
    // TODO: Note about taking care when mutating the reference
    #[inline]
    pub fn load(&self) -> Option<&'a mut T> {
        unsafe { self.cell.load().as_mut() }
    }

    /// Loads the mutable reference, without checking for None.
    #[inline]
    pub unsafe fn load_unchecked(&self) -> &'a mut T {
        &mut *self.cell.load()
    }

    /// Stores a mutable reference.
    #[inline]
    pub fn store(&self, value: Option<&'a mut T>) {
        unsafe { self.cell.store(mem::transmute(value)) }
    }
}

unsafe impl<'a, T: Thin + ?Sized> Send for AtomicMut<'a, T> {}
unsafe impl<'a, T: Thin + ?Sized> Sync for AtomicMut<'a, T> {}
