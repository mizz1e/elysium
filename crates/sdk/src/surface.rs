use crate::vtable_validate;
use cake::ffi::VTablePad;

#[repr(C)]
pub struct VTable {
    _pad0: VTablePad<66>,
    unlock_cursor: unsafe extern "thiscall" fn(surface: *const Surface),
}

vtable_validate! {
    unlock_cursor => 66,
}

#[repr(C)]
pub struct Surface {
    vtable: &'static VTable,
}

impl Surface {
    #[inline]
    pub fn unlock_cursor(&self) {
        unsafe { (self.vtable.unlock_cursor)(self) }
    }
}
