use super::vtable_validate;
use cake::ffi::VTablePad;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<11>,
    enable_input: unsafe extern "thiscall" fn(this: *const InputSystem, enable: bool),
    _pad1: VTablePad<27>,
    reset_input_state: unsafe extern "thiscall" fn(this: *const InputSystem),
    _pad2: VTablePad<17>,
    cursor_visible: unsafe extern "thiscall" fn(this: *const InputSystem, visible: bool),
}

vtable_validate! {
    enable_input => 11,
    reset_input_state => 39,
    cursor_visible => 57,
}

/// Input System interface.
#[repr(C)]
pub struct InputSystem {
    vtable: &'static VTable,
}

impl InputSystem {
    #[inline]
    pub fn enable_input(&self, enable: bool) {
        unsafe { (self.vtable.enable_input)(self, enable) }
    }

    #[inline]
    pub fn reset_input_state(&self) {
        unsafe { (self.vtable.reset_input_state)(self) }
    }

    #[inline]
    pub fn cursor_visible(&self, enable: bool) {
        unsafe { (self.vtable.cursor_visible)(self, enable) }
    }
}
