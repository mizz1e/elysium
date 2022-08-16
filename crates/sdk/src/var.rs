use crate::UtlVec;
use cake::ffi::BytePad;

#[repr(C)]
pub struct VarEntry {
    pub kind: u16,
    pub need_to_interpolate: u16,
    _pad0: BytePad<8>,
}

#[repr(C)]
pub struct VarMap {
    pub entries: UtlVec<VarEntry>,
    pub interpolated_entries: i32,
}
