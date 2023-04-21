use std::ptr;
use std::ptr::NonNull;

/// The size of a page.
const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
const PAGE_MASK: usize = !(PAGE_SIZE - 1);

/// Read, write, and execute.
const RWX: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

/// Read, and execute.
const RX: i32 = libc::PROT_READ | libc::PROT_EXEC;

/// Set protection for the page of the given pointer.
unsafe fn set_protection<T>(ptr: *const T, protection: i32) {
    let page = ptr.map_addr(|addr| addr & PAGE_MASK);

    libc::mprotect(page as *mut libc::c_void, PAGE_SIZE, protection);
}

/// Performs [`ptr::replace`](ptr::replace) for a pointer existing in read-only memory.
///
/// # Safety
///
/// See [`ptr::replace`](ptr::replace).
pub unsafe fn replace_protected<T>(dst: *const T, src: T) -> T {
    set_protection(dst, RWX);

    let old = ptr::replace(dst as *mut T, src);

    set_protection(dst, RX);

    old
}

/// A function pointer.
pub type FnPtr = unsafe extern "C" fn();

/// A virtual table.
#[derive(Clone, Copy, Debug)]
pub enum VTable {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Object {
    vtable: *mut VTable,
}

/// A type-erased pointer to a source engine object.
#[derive(Clone, Copy, Debug)]
pub struct Ptr {
    #[cfg(debug_assertions)]
    label: &'static str,

    ptr: NonNull<u8>,
}

impl Ptr {
    /// Construct a new pointer to a source engine object.
    #[must_use]
    pub fn new(label: &'static str, ptr: *mut u8) -> Option<Ptr> {
        let ptr = NonNull::new(ptr)?;

        #[cfg(debug_assertions)]
        {
            Some(Ptr { label, ptr })
        }

        #[cfg(not(debug_assertions))]
        {
            let _label = label;

            Some(Ptr { ptr })
        }
    }

    /// Returns a raw pointer to the object.
    #[must_use]
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Obtain a pointer to a object's virtual table.
    ///
    /// # Safety
    ///
    /// - The pointed to type must contain a virtual table.
    /// - See [`ptr::read`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
    #[must_use]
    pub unsafe fn vtable_ptr(&self) -> *mut VTable {
        let object = (self.as_ptr() as *mut Object).read_unaligned();

        object.vtable
    }

    /// Obtain a pointer to an entry within a object's virtual table.
    ///
    /// # Safety
    ///
    /// - `index` must be a valid virtual table index.
    /// - See [`vtable_ptr`](Ptr::vtable_ptr).
    #[must_use]
    pub unsafe fn vtable_index<T>(&self, index: usize) -> *mut T {
        let vtable = self.vtable_ptr() as *mut FnPtr;

        vtable.add(index) as *mut T
    }

    /// Obtain a pointer to an entry within a object's virtual table.
    ///
    /// # Safety
    ///
    /// - `index` must be a valid virtual table index.
    /// - See [`vtable_ptr`](Ptr::vtable_ptr).
    #[must_use]
    pub unsafe fn vtable_entry<T>(&self, index: usize) -> T {
        self.vtable_index::<T>(index).read()
    }

    /// Replace a entry within a object's virtual table.
    ///
    /// # Safety
    ///
    /// - The memory page of which the virtual table entry resides is not needed for execution at
    /// the time of writing the new entry.
    /// - See [`vtable_index`](Ptr::vtable_index).
    pub unsafe fn vtable_replace<T: Copy>(&self, index: usize, f: T) -> T {
        let ptr = self.vtable_index::<T>(index);
        let old = replace_protected(ptr, f);

        #[cfg(debug_assertions)]
        {
            tracing::trace!("replaced virtual index {index} for {:?}", self.label);
        }

        old
    }

    /// Calculates the offset from the base of this object. The provided `count` is in bytes.
    ///
    /// # Safety
    ///
    /// See [`ptr::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
    #[must_use]
    pub unsafe fn byte_offset<T>(&self, offset: usize) -> *mut T {
        let this = self.as_ptr() as *mut u8;

        this.add(offset) as *mut T
    }
}

unsafe impl Send for Ptr {}
unsafe impl Sync for Ptr {}
