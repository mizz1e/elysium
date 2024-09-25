use crate::{PlayerFlag, Tick, Time};
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
    pub(super) fn new(offset: usize) -> Self {
        Self {
            offset,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn addr<U>(self, class: *mut U) -> *mut u8 {
        class.cast::<u8>().add(self.offset)
    }

    unsafe fn _read<U, V>(self, class: *const V) -> U {
        class
            .cast::<u8>()
            .add(self.offset)
            .cast::<U>()
            .read_unaligned()
    }

    unsafe fn _write<U, V>(self, class: *mut V, value: U) {
        class
            .cast::<u8>()
            .add(self.offset)
            .cast::<U>()
            .write_unaligned(value);
    }

    fn cast<U>(self) -> Var<U> {
        unsafe { mem::transmute(self) }
    }
}

macro_rules! vars {
    ($($ty:ty,)*) => {$(
        impl Var<$ty> {

            pub unsafe fn read<T>(self, class: *const T) -> $ty {
                self._read(class)
            }


            pub unsafe fn write<T>(self, class: *mut T, value: $ty) {
                self._write(class, value);
            }
        }
    )*}
}

vars! { bool, f32, i32, u32, PlayerFlag, Vec3, }

impl Var<Duration> {
    pub unsafe fn read<T>(self, class: *const T) -> Duration {
        Duration::from_secs_f32(self.cast::<f32>().read(class))
    }

    pub unsafe fn write<T>(self, class: *mut T, value: Duration) {
        self.cast::<f32>().write(class, value.as_secs_f32());
    }
}

impl Var<Tick> {
    pub unsafe fn read<T>(self, class: *const T) -> Tick {
        Tick(self.cast::<u32>().read(class))
    }
}

impl Var<Time> {
    pub unsafe fn read<T>(self, class: *const T) -> Time {
        Time(self.cast::<Duration>().read(class))
    }
}

impl Var<Option<Box<OsStr>>> {
    pub unsafe fn read<T>(self, class: *const T) -> Option<Box<OsStr>> {
        let string = class.cast::<u8>().add(self.offset).cast::<ffi::c_char>();
        let string = CStr::from_ptr(string).to_bytes();

        if string.is_empty() {
            return None;
        }

        Some(Box::from(OsStr::from_bytes(string)))
    }
}

impl<T> Clone for Var<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Var<T> {}

impl<T> fmt::Debug for Var<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&ptr::with_exposed_provenance::<()>(self.offset), fmt)
    }
}
