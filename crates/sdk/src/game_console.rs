use std::{ffi, ptr};

#[repr(C)]
struct VTable {
    drop: unsafe extern "thiscall" fn(this: *const GameConsole),
    show: unsafe extern "thiscall" fn(this: *const GameConsole),
    init: unsafe extern "thiscall" fn(this: *const GameConsole),
    hide: unsafe extern "thiscall" fn(this: *const GameConsole),
    clear: unsafe extern "thiscall" fn(this: *const GameConsole),
    is_console_visible: unsafe extern "thiscall" fn(this: *const GameConsole) -> bool,
    set_parent: unsafe extern "thiscall" fn(this: *const GameConsole, parent: ffi::c_int),
}

#[repr(C)]
pub struct GameConsole {
    vtable: &'static mut VTable,
}

impl GameConsole {
    #[inline]
    pub fn show_address(&mut self) -> *mut *const () {
        ptr::addr_of_mut!(self.vtable.show).cast()
    }
}
