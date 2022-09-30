use crate::state::material;
use core::ffi;
use elysium_sdk::material::{Material, Materials};
use elysium_sdk::Vdf;

pub unsafe extern "C" fn create_material(
    materials: &Materials,
    name_pointer: *const ffi::c_char,
    vdf: Option<&Vdf>,
) -> Option<&'static mut Material> {
    let name = cake::ffi::CUtf8Str::from_ptr(name_pointer).as_str();

    materials.from_vdf(name, vdf)
}
