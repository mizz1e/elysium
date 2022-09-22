use super::Class;
use core::mem;

/// An iterator over classes.
pub struct ClassIter<'a> {
    pub(crate) iter: Option<&'a Class>,
}

impl<'a> Iterator for ClassIter<'a> {
    type Item = &'a Class;

    #[inline]
    fn next(&mut self) -> Option<&'a Class> {
        let next = unsafe { self.iter?.next.as_ref() };

        mem::replace(&mut self.iter, next)
    }
}
