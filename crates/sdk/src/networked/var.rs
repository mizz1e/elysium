use crate::entity::PlayerFlags;
use bevy::math::Vec3;
use core::marker::PhantomData;
use core::time::Duration;
use core::{ffi, fmt, mem, ptr};
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;

#[repr(transparent)]
pub struct Var<T> {
    pub(super) offset: usize,
    _phantom: PhantomData<T>,
}

impl<T> Var<T> {
    #[inline]
    pub(super) fn new(offset: usize) -> Self {
        Self {
            offset,
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub unsafe fn addr<U>(self, class: *const U) -> *const () {
        self.add(class).cast()
    }

    #[inline]
    unsafe fn add<U>(self, class: *const U) -> *const T {
        class.byte_add(self.offset).cast()
    }

    #[inline]
    unsafe fn add_mut<U>(self, class: *mut U) -> *mut T {
        class.byte_add(self.offset).cast()
    }

    #[inline]
    fn cast<U>(self) -> Var<U> {
        unsafe { mem::transmute(self) }
    }
}

impl Var<bool> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> bool {
        self.add(class).read_unaligned()
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: bool) {
        self.add_mut(class).write_unaligned(value)
    }
}

impl Var<f32> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> f32 {
        self.add(class).read_unaligned()
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: f32) {
        self.add_mut(class).write_unaligned(value)
    }
}

impl Var<i32> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> i32 {
        self.add(class).read_unaligned()
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: i32) {
        self.add_mut(class).write_unaligned(value)
    }
}

impl Var<u32> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> u32 {
        self.add(class).read_unaligned()
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: u32) {
        self.add_mut(class).write_unaligned(value)
    }
}

impl Var<Duration> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> Duration {
        Duration::from_secs_f32(self.cast::<f32>().read(class))
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: Duration) {
        let value = value.as_secs_f32();

        self.cast::<f32>().write(class, value);
    }
}

impl Var<PlayerFlags> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> PlayerFlags {
        PlayerFlags::from_i32(self.cast::<i32>().read(class))
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: PlayerFlags) {
        let value = value.to_i32();

        self.cast::<i32>().write(class, value);
    }
}

impl Var<Vec3> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> Vec3 {
        self.add(class).read_unaligned()
    }

    #[inline]
    pub unsafe fn write<T>(self, class: *mut T, value: Vec3) {
        self.add_mut(class).write_unaligned(value)
    }
}

impl Var<Box<OsStr>> {
    #[inline]
    pub unsafe fn read<T>(self, class: *const T) -> Box<OsStr> {
        let pointer = self.add(class).cast::<ffi::c_char>();
        let bytes = CStr::from_ptr(pointer).to_bytes();
        let os_str = OsStr::from_bytes(bytes);

        Box::from(os_str)
    }
}

impl<T> Clone for Var<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Var<T> {}

impl<T> fmt::Debug for Var<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&ptr::from_exposed_addr::<()>(self.offset), fmt)
    }
}
