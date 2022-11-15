use crate::{vtable_validate, Command, View};
use cake::ffi::VTablePad;
use core::ptr;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<19>,
    override_view: unsafe extern "C" fn(this: *const ClientMode, view: *const View),
    _pad1: VTablePad<5>,
    create_move: unsafe extern "C" fn(
        this: *const ClientMode,
        input_sample_time: f32,
        command: *mut Command,
    ) -> bool,
}

vtable_validate! {
    override_view => 19,
    create_move => 25,
}

#[repr(C)]
pub struct ClientMode {
    vtable: &'static VTable,
}

impl ClientMode {
    #[inline]
    pub fn create_move_address(&self) -> *const u8 {
        ptr::addr_of!(self.vtable.create_move).cast()
    }

    #[inline]
    pub fn override_view_address(&self) -> *const u8 {
        ptr::addr_of!(self.vtable.override_view).cast()
    }
}
