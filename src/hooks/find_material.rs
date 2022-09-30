use core::ffi;
use elysium_sdk::material::{Material, Materials};

pub unsafe extern "C" fn find_material(
    materials: &Materials,
    name_pointer: *const ffi::c_char,
    group_pointer: *const ffi::c_char,
    _complain: bool,
    _complain_prefix: *const ffi::c_char,
) -> Option<&'static mut Material> {
    let name = cake::ffi::CUtf8Str::from_ptr(name_pointer).as_str();
    let group = cake::ffi::CUtf8Str::from_ptr(group_pointer).as_str();

    println!("{name:?} {group:?}");

    materials.find(name, group)
}
