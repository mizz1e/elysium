use elysium_sdk::materials::Material;

const NEW: Materials = Materials { gold: None };

pub struct Materials {
    pub gold: Option<&'static Material>,
}

impl Materials {
    #[inline]
    pub const fn new() -> Self {
        NEW
    }
}
