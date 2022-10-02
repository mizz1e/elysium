#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Shader {
    Lit,
    Unlit,
}

impl Shader {
    #[inline]
    pub const fn base(&self) -> &'static str {
        match self {
            Shader::Lit => "VertexLitGeneric\0",
            Shader::Unlit => "UnlitGeneric\0",
        }
    }
}

macro_rules! materials {
    ($($variant:ident: $shader:ident = $vdf:expr,)*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum Kind {
            $($variant,)*
        }

        impl Kind {
            /// Returns the material this shader uses.
            #[inline]
            pub const fn shader(&self) -> Shader {
                match self {
                    $(Kind::$variant => Shader::$shader,)*
                }
            }

            /// VDF/KeyValues passed to the second argument of
            /// [`Vdf::from_bytes`](crate::Vdf::from_bytes).
            #[inline]
            pub const fn vdf(&self) -> Option<&'static str> {
                match self {
                    $(Kind::$variant => $vdf,)*
                }
            }
        }
    }
}

materials! {
    Normal: Lit = None,
    Flat: Unlit = None,
    Chrome: Lit = Some("
        $envmap env_cubemap
    \0"),
    Glow: Lit = Some("
        $additive 1
        $envmap models/effects/cube_white
        $envmapfresnel 1
        $alpha 0.8
    \0"),
    Pearlescent: Lit = Some("
        $ambientonly 1
        $phong 1
        $pearlescent 3
        $basemapalphaphongmask 1
    \0"),
    Metallic: Lit = Some("
        $basetexture white
        $envmap env_cubemap
        $envmapcontrast 1
        $flat 1
        $halfambert 1
        $ignorez 0
        $model 1
        $nocull 0
        $nofog 1
        $normalmapalphaenvmapmask 1
        $selfillum 1
        $znearer 0
    \0"),
    Animated: Lit = Some("
        $basetexture dev/zone_warning
        $envmap editor/cube_vertigo
        $envmapcontrast 1
        $envmaptint [0.7 0.7 0.7]
        proxies {
            texturescroll {
                texturescollvar $basetexturetransform
                texturescrollrate 0.6
                texturescrollangle 90
            }
        }
    \0"),
    Platinum: Lit = Some("
        $basetexture models/player/ct_fbi/ct_fbi_glass
        $envmap env_cubemap
        $envmaptint [0.4 0.6 0.7]
    \0"),
    Glass: Lit = Some("
        $additive 1
        $basetexture detail/dt_metal1
        $color [0.05 0.05 0.05]
        $envmap editor/cube_vertigo
    \0"),
    Crystal: Lit = Some("
        $basetexture black
        $bumpmap effects/flat_normal
        $envmap models/effects/crystal_cube_vertigo_hdr
        $envmapfresnel 0
        $phong 1
        $phongboost 2
        $phongexponent 16
        $phongtint [0.2 0.35 0.6]
        $translucent 1
    \0"),
    Silver: Lit = Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [0.05 0.05 0.05]
        $envmap editor/cube_vertigo
        $envmapfresnel 0.6
        $envtintmap [0.2 0.2 0.2]
        $phong 1
        $phongboost 2
        $phongexponent 8
        $phongfresnelranges [0.7 0.8 1]
        $phongtint [0.8 0.9 1]
    \0"),
    Gold: Lit = Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [0.18 0.15 0.06]
        $envmap editor/cube_vertigo
        $envmapfresnel 0.6
        $envtintmap [0.6 0.5 0.2]
        $phong 1
        $phongboost 6
        $phongdisablehalflambert 1
        $phongexponent 128
        $phongfresnelranges [0.7 0.8 1]
        $phongtint [0.6 0.5 0.2]
    \0"),
    Plastic: Lit = Some("
        $additive 1
        $basetexture black
        $bumpmap models/inventory_items/trophy_majors/matte_metal_normal
        $envmap editor/cube_vertigo
        $envmapfresnel 1
        $normalmapalphaenvmapmask 1
        $phong 1
        $phongboost 20
        $phongdisablehalflambert 1
        $phongexponent 3000
        $phongfesnelranges [0.1 0.4 1]
        $phongtint [0.8 0.9 1]
    \0"),
}
