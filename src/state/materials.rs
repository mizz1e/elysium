use elysium_sdk::material::{Material, MaterialKind};
use elysium_sdk::MaterialSystem;

const NEW: Materials = Materials {
    flat: None,
    glow: None,
};

pub struct Materials {
    pub flat: Option<&'static mut Material>,
    pub glow: Option<&'static mut Material>,
}

impl Materials {
    #[inline]
    pub const fn new() -> Self {
        NEW
    }

    #[inline]
    pub fn get(&mut self, kind: MaterialKind, system: &MaterialSystem) -> &'static mut Material {
        let material = match kind {
            MaterialKind::Flat => &mut self.flat,
            MaterialKind::Glow => &mut self.glow,
            _ => unimplemented!(),
        };

        let value: *mut &'static mut Material = material
            .get_or_insert_with(|| system.from_kind(kind).expect("failed to create material"));

        let material = unsafe { &mut **(value as *mut *mut Material) };

        if kind == MaterialKind::Glow {
            const PURPLE: [f32; 4] = [0.4, 0.0, 1.0, 0.4];

            material.set_rgba(PURPLE);
        }

        material
    }
}
