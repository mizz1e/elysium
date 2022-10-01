use crate::state::material;
use core::ffi;
use elysium_sdk::material::{BorrowedMaterial, Group, Material, Materials};

pub unsafe extern "C" fn find_material(
    materials: &Materials,
    name_pointer: *const ffi::c_char,
    group_pointer: *const ffi::c_char,
    _complain: bool,
    _complain_prefix: *const ffi::c_char,
) -> Option<&'static mut Material> {
    let name = cake::ffi::CUtf8Str::from_ptr(name_pointer).as_str();
    let group = if group_pointer.is_null() {
        None
    } else {
        Some(Group::from_bytes(
            cake::ffi::CBytes::from_ptr(group_pointer).as_bytes(),
        ))
    };

    let state = crate::State::get();

    //println!("{name:?} {group:?}");

    match group {
        Some(Group::StaticProp | Group::World) => {
            if let Some(material) = materials.find(name, group) {
                let borrowed = &mut *(material as *mut _);

                state
                    .world
                    .as_mut()
                    .unwrap()
                    .insert(BorrowedMaterial::from_mut(borrowed));

                return Some(material);
            } else {
                return None;
            }
        }
        _ => {}
    }

    if name.contains("blur") {
        if let Some(material) = materials.find(name, group) {
            println!("BLUR {name:?} {group:?}");

            let borrowed = &mut *(material as *mut _);

            state
                .blur
                .as_mut()
                .unwrap()
                .insert(BorrowedMaterial::from_mut(borrowed));

            return Some(material);
        } else {
            return None;
        }
    }

    if name.starts_with("compositing_material")
        || name.starts_with("models/props")
        || name.starts_with("models/weapons")
        || name.contains("vgui")
    {
        return materials.find(name, group);
    }

    if name.contains("blood") {
        println!("BLOOD {name:?}");
        return material::BLOOD.load();
    }

    if name.contains("muzzleflash") {
        println!("MUZZLE FLASH {name:?}");
        return material::MUZZLE_FLASH.load();
    }

    if name.contains("vistasmoke") {
        println!("SMOKE {name:?}");
        return material::SMOKE.load();
    }

    if name.contains("fire") {
        println!("FIRE {name:?}");
        return material::FIRE.load();
    }

    if name.contains("decal") {
        println!("DECAL {name:?}");
        return material::DECAL.load();
    }

    materials.find(name, group)
}
