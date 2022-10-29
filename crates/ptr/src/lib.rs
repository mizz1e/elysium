use std::pin::Pin;
use std::{fmt, mem, ops};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Ptr<'a, T>(Pin<&'a T>);

#[repr(transparent)]
pub struct PtrMut<'a, T>(Pin<&'a mut T>);

impl<'a, T> Ptr<'a, T> {
    pub fn new(value: &'a T) -> Ptr<'a, T> {
        Ptr(unsafe { Pin::new_unchecked(value) })
    }

    pub unsafe fn cast<U>(self) -> Ptr<'a, U> {
        mem::transmute(self)
    }
}

impl<'a, T> PtrMut<'a, T> {
    pub fn new(value: &'a mut T) -> PtrMut<'a, T> {
        PtrMut(unsafe { Pin::new_unchecked(value) })
    }

    pub unsafe fn cast<U>(self) -> PtrMut<'a, U> {
        mem::transmute(self)
    }

    pub unsafe fn clone(self: &PtrMut<'a, T>) -> PtrMut<'a, T> {
        mem::transmute_copy(self)
    }
}

impl<'a, T> ops::Deref for Ptr<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T> ops::Deref for PtrMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: Unpin> ops::DerefMut for PtrMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for PtrMut<'a, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}
