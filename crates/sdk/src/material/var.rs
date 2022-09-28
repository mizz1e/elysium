use crate::{ffi, vtable_validate};
use cake::ffi::VTablePad;
use std::ffi::OsStr;

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/materialsystem/imaterialvar.h
#[repr(C)]
struct VTable {
    _pad0: VTablePad<3>,
    set_f32: unsafe extern "thiscall" fn(this: *mut Var, value: f32),
    set_i32: unsafe extern "thiscall" fn(this: *mut Var, value: i32),
    set_string: unsafe extern "thiscall" fn(this: *mut Var, value: *const libc::c_char),
    _pad1: VTablePad<6>,
    set_vec3: unsafe extern "thiscall" fn(this: *mut Var, x: f32, y: f32, z: f32),
    set_vec4: unsafe extern "thiscall" fn(this: *mut Var, x: f32, y: f32, z: f32, w: f32),
}

vtable_validate! {
    set_f32 => 3,
    set_i32 => 4,
    set_string => 5,
    set_vec3 => 12,
    set_vec4 => 13,
}

#[repr(C)]
pub struct Var {
    vtable: &'static VTable,
}

impl Var {
    #[inline]
    pub fn set_f32(&mut self, value: f32) {
        unsafe { (self.vtable.set_f32)(self, value) }
    }

    #[inline]
    pub fn set_i32(&mut self, value: i32) {
        unsafe { (self.vtable.set_i32)(self, value) }
    }

    #[inline]
    pub fn set_string<S>(&mut self, string: S)
    where
        S: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(string, |string| unsafe {
            (self.vtable.set_string)(self, string.as_ptr());
        });
    }

    #[inline]
    pub fn set_vec3(&mut self, value: [f32; 3]) {
        let [r, g, b] = value;

        unsafe { (self.vtable.set_vec3)(self, r, g, b) }
    }

    #[inline]
    pub fn set_vec4(&mut self, value: [f32; 4]) {
        let [r, g, b, w] = value;

        unsafe { (self.vtable.set_vec4)(self, r, g, b, w) }
    }
}
