use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{ffi, str};

use elysium_sdk::material::{BorrowedMaterial, Kind, Material, Materials};
use elysium_sdk::Vdf;

use crate::state;

fn init_materials(
    material_system: &Materials,
    materials: &[(&str, Kind, &elysium_sdk::AtomicMut<Material>)],
) {
    for (name, kind, material) in materials.iter() {
        material.store(Some(material_system.from_kind(name, *kind).unwrap()));
    }
}

static MATERIALS_INIT: AtomicBool = AtomicBool::new(false);

pub unsafe extern "C" fn create_material(
    materials: &Materials,
    name_pointer: *const ffi::c_char,
    vdf: Option<&Vdf>,
) -> Option<&'static mut Material> {
    if MATERIALS_INIT
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        init_materials(
            materials,
            &[
                ("elysium/blood\0", Kind::Glow, &state::material::BLOOD),
                ("elysium/decal\0", Kind::Glow, &state::material::DECAL),
                ("elysium/fire\0", Kind::Glow, &state::material::FIRE),
                ("elysium/impact\0", Kind::Glow, &state::material::IMPACT),
                (
                    "elysium/muzzle_flash\0",
                    Kind::Glow,
                    &state::material::MUZZLE_FLASH,
                ),
                ("elysium/path\0", Kind::Glow, &state::material::PATH),
                ("elysium/particle\0", Kind::Glow, &state::material::PARTICLE),
                ("elysium/prop\0", Kind::Glow, &state::material::PROP),
                ("elysium/smoke\0", Kind::Glow, &state::material::SMOKE),
                ("elysium/tree\0", Kind::Glow, &state::material::TREE),
                ("elysium/flat\0", Kind::Flat, &state::material::FLAT),
                ("elysium/glow\0", Kind::Glow, &state::material::GLOW),
            ],
        );
    }

    let name = CStr::from_ptr(name_pointer).to_bytes();
    let name = std::str::from_utf8(name).unwrap();
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
