use std::hash::{Hash, Hasher};
use std::{ffi, fmt};

/// Material identifier.
#[derive(Clone, Eq)]
pub struct MaterialId {
    pub(crate) id: ffi::c_ushort,

    #[cfg(debug_assertions)]
    pub(crate) context: Box<str>,
}

impl fmt::Debug for MaterialId {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.id, fmt)
    }
}

impl PartialEq for MaterialId {
    #[inline]
    fn eq(&self, other: &MaterialId) -> bool {
        self.id == other.id
    }
}

impl Hash for MaterialId {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        0xFF_u8.hash(state);
    }
}
