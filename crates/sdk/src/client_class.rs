use super::dt_recv::RecvTable;
use std::ffi;

// returns client networkable
pub type CreateClientClass =
    unsafe extern "C" fn(entity: ffi::c_int, serial: ffi::c_int) -> *mut ffi::c_void;

// returns client networkable
pub type CreateEvent = unsafe extern "C" fn() -> *mut ffi::c_void;

/// Networkable client class.
///
/// See [`public/client_class.h`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/client_class.h).
#[repr(C)]
pub struct ClientClass {
    pub create: CreateClientClass,
    pub create_event: CreateEvent,
    pub network_name: *const ffi::c_char,
    pub recv_table: *const RecvTable,
    pub next: *mut ClientClass,
    pub id: ffi::c_int,
    pub map_class_name: *const ffi::c_char,
}
