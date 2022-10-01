use crate::state::material;
use core::ffi;
use elysium_sdk::material::{BorrowedMaterial, Material, Materials};
use elysium_sdk::Vdf;

pub unsafe extern "C" fn create_material(
    materials: &Materials,
    name_pointer: *const ffi::c_char,
    vdf: Option<&Vdf>,
) -> Option<&'static mut Material> {
    let name = cake::ffi::CUtf8Str::from_ptr(name_pointer).as_str();
    let state = crate::State::get();

    if name.contains("blur") {
        if let Some(material) = materials.from_vdf(name, vdf) {
            println!("BLUR {name:?}");

            let borrowed = &mut *(material as *mut _);

            state
                .blur_static
                .as_mut()
                .unwrap()
                .insert(BorrowedMaterial::from_mut(borrowed));

            return Some(material);
        } else {
            return None;
        }
    }

    materials.from_vdf(name, vdf)
}
