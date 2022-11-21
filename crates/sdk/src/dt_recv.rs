//! `public/dt_recv.h`

use bevy::prelude::*;
use std::{ffi, slice};

// `base` is the structure the data table variable is in
// `field` is the field the data table variable is stored in
pub type RecvProxy =
    unsafe extern "C" fn(data: *const RecvProxyData, base: *mut u8, field: *mut u8);

pub type RecvArrayLenProxy = unsafe extern "C" fn(
    data: *const ffi::c_void,
    object_id: ffi::c_int,
    current_array_len: ffi::c_int,
);

// `data` refers to the object + table offset
// `out` refers to where to unpack the data table into
pub type RecvVarProxy = unsafe extern "C" fn(
    prop: *const RecvProp,
    out: *const *const ffi::c_void,
    data: *const ffi::c_void,
    object_id: ffi::c_int,
);

#[derive(Debug)]
#[repr(C)]
pub enum PropKind {
    I32 = 0,
    F32 = 1,
    Vec3 = 2,
    Vec2 = 3,
    String = 4,
    Array = 5,
    DataTable = 6,
    I64 = 7,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union PropValue {
    pub as_f32: f32,
    pub as_i32: i32,
    pub as_cstr: *const ffi::c_char,
    pub as_data: *const ffi::c_void,
    pub as_vec3: Vec3,
    pub as_i64: i64,
}

#[repr(C)]
pub struct PropVariant {
    pub value: PropValue,
    pub kind: PropKind,
}

#[repr(C)]
pub struct RecvProp {
    pub name: *const ffi::c_char,
    pub kind: PropKind,
    pub flags: ffi::c_int,
    pub string_capacity: ffi::c_int,
    pub inside_array: bool,
    pub extra_data: *const ffi::c_void,
    pub array_prop: *mut RecvProp,
    pub array_len_proxy: RecvArrayLenProxy,
    pub proxy: *const u8,
    pub data_table_proxy: RecvVarProxy,
    pub data_table: *mut RecvTable,
    pub offset: ffi::c_int,
    pub element_stride: ffi::c_int,
    pub elements: ffi::c_int,
    pub parent_array_prop_name: *const ffi::c_char,
}

#[repr(C)]
pub struct RecvTable {
    pub props: *mut RecvProp,
    pub props_len: ffi::c_int,
    pub decoder: *const ffi::c_void,
    pub name: *const ffi::c_char,
    pub initialized: bool,
    pub in_main_list: bool,
}

#[repr(C)]
pub struct RecvProxyData {
    pub prop: *const RecvProp,
    pub value: PropVariant,
    pub element: ffi::c_int,
    pub object_id: ffi::c_int,
}

impl RecvTable {
    pub(crate) unsafe fn props(&self) -> &[RecvProp] {
        slice::from_raw_parts(self.props, self.props_len as usize)
    }
}
