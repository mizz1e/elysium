use crate::state;
use core::mem::MaybeUninit;
use core::ptr;
use elysium_sdk::materials::{Material, MaterialKind, Materials};
use elysium_sdk::model::ModelRender;
use elysium_sdk::Interfaces;
use elysium_sdk::Vdf;

fn vdf_init(vdf: *mut Vdf, base: *const u8) {
    state::hooks::vdf_init(vdf.cast(), base, 0, 0);
}

fn vdf_from_bytes(vdf: *mut Vdf, name: *const u8, bytes: *const u8) {
    state::hooks::vdf_from_bytes(
        vdf.cast(),
        name,
        bytes,
        ptr::null(),
        ptr::null(),
        ptr::null(),
    );
}

fn create_material(materials: &Materials, material: MaterialKind) -> *const Material {
    let mut vdf: MaybeUninit<Vdf> = MaybeUninit::uninit();
    let vdf = vdf.as_mut_ptr();

    // Offsets::initKeyValues(keyValues, materialType, 0, 0);
    vdf_init(vdf, material.base_ptr());
    // Offsets::loadFromBuffer(keyValues, materialName, material, nullptr, nullptr, nullptr);
    vdf_from_bytes(vdf, material.name_ptr(), material.vdf_ptr());

    // return Interfaces::materialSystem->CreateMaterial(materialName, keyValues);
    materials
        .create(material.name(), vdf.cast())
        .cast::<Material>()
}

pub unsafe extern "C" fn draw_model(
    this: *const u8,
    context: *const u8,
    state: *const u8,
    info: *const u8,
    bone_to_world: *const u8,
) {
    frosting::println!();

    let interfaces = &*state::interfaces().cast::<Interfaces>();
    let materials = &*interfaces.material.cast::<Materials>();
    let model_render = &*interfaces.model_render.cast::<ModelRender>();

    let material = &*create_material(materials, MaterialKind::Gold);

    model_render.override_material(material);
    state::hooks::draw_model(this, context, state, info, bone_to_world);
    model_render.reset_material();
}
