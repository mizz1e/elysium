#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MaterialFlag(pub i32);

impl MaterialFlag {
    pub const DEBUG: Self = Self(1 << 0);
    pub const NO_DEBUG_OVERRIDE: Self = Self(1 << 1);
    pub const NO_DRAW: Self = Self(1 << 2);
    pub const USE_IN_FILLRATE_MODE: Self = Self(1 << 3);

    pub const VERTEXCOLOR: Self = Self(1 << 4);
    pub const VERTEXALPHA: Self = Self(1 << 5);
    pub const SELFILLUM: Self = Self(1 << 6);
    pub const ADDITIVE: Self = Self(1 << 7);
    pub const ALPHATEST: Self = Self(1 << 8);

    pub const ZNEARER: Self = Self(1 << 10);
    pub const MODEL: Self = Self(1 << 11);
    pub const FLAT: Self = Self(1 << 12);
    pub const NO_CULL: Self = Self(1 << 13);
    pub const NO_FOG: Self = Self(1 << 14);
    pub const IGNORE_Z: Self = Self(1 << 15);
    pub const DECAL: Self = Self(1 << 16);
    pub const ENVMAPSPHERE: Self = Self(1 << 17);

    pub const ENVMAPCAMERASPACE: Self = Self(1 << 19);
    pub const BASEALPHAENVMAPMASK: Self = Self(1 << 20);
    pub const TRANSLUCENT: Self = Self(1 << 21);
    pub const NORMALMAPALPHAENVMAPMASK: Self = Self(1 << 22);
    pub const NEEDS_SOFTWARE_SKINNING: Self = Self(1 << 23);
    pub const OPAQUETEXTURE: Self = Self(1 << 24);
    pub const ENVMAPMODE: Self = Self(1 << 25);
    pub const SUPPRESS_DECALS: Self = Self(1 << 26);
    pub const HALFLAMBERT: Self = Self(1 << 27);
    pub const WIREFRAME: Self = Self(1 << 28);
    pub const ALLOWALPHATOCOVERAGE: Self = Self(1 << 29);
    pub const ALPHA_MODIFIED_BY_PROXY: Self = Self(1 << 30);
    pub const VERTEXFOG: Self = Self(1 << 31);
}
