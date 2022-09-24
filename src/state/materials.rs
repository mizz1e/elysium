use elysium_sdk::material::Material;

const NEW: Materials = Materials {
    flat: None,
    glow: None,
};

pub struct Materials {
    pub flat: Option<&'static Material>,
    pub glow: Option<&'static Material>,
}

impl Materials {
    #[inline]
    pub const fn new() -> Self {
        NEW
    }
}
