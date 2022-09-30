#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Shader {
    Lit,
    Unlit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Kind {
    Flat,
    Chrome,
    Glow,
    Pearlescent,
    Metallic,
    Animated,
    Platinum,
    Glass,
    Crystal,
    Silver,
    Gold,
    Plastic,
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
    ($($variant:ident => ($name:literal, $shader:ident, $vdf:expr)),*) => {
        impl Kind {
            /// Material name passed to `Material::new`.
            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(Kind::$variant => $name,)*
                }
            }

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
    Flat => ("flat\0", Unlit, None),
    Chrome => ("chrome\0", Lit, Some("
        $envmap env_cubemap
    \0")),
    Glow => ("glow\0", Lit, Some("
        $additive 1
        $envmap models/effects/cube_white
        $envmapfresnel 1
        $alpha .8
    \0")),
    Pearlescent => ("pearlescent\0", Lit, Some("
        $ambientonly 1
        $phong 1
        $pearlescent 3
        $basemapalphaphongmask 1
    \0")),
    Metallic => ("metallic\0", Lit, Some("
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
    \0")),
    Animated => ("animated\0", Lit, Some("
        $basetexture dev/zone_warning
        $envmap editor/cube_vertigo
        $envmapcontrast 1
        $envmaptint [.7 .7 .7]
        proxies {
            texturescroll {
                texturescollvar $basetexturetransform
                texturescrollrate 0.6
                texturescrollangle 90
            }
        }
    \0")),
    Platinum => ("platinum\0", Lit, Some("
        $basetexture models/player/ct_fbi/ct_fbi_glass
        $envmap env_cubemap
        $envmaptint [.4 .6 .7]
    \0")),
    Glass => ("glass", Lit, Some("
        $additive 1
        $basetexture detail/dt_metal1
        $color [.05 .05 .05]
        $envmap editor/cube_vertigo
    \0")),
    Crystal => ("crystal\0", Lit, Some("
        $basetexture black
        $bumpmap effects/flat_normal
        $envmap models/effects/crystal_cube_vertigo_hdr
        $envmapfresnel 0
        $phong 1
        $phongboost 2
        $phongexponent 16
        $phongtint [.2 .35 .6]
        $translucent 1
    \0")),
    Silver => ("silver\0", Lit, Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [.05 .05 .05]
        $envmap editor/cube_vertigo
        $envmapfresnel .6
        $envtintmap [.2 .2 .2]
        $phong 1
        $phongboost 2
        $phongexponent 8
        $phongfresnelranges [.7 .8 1]
        $phongtint [.8 .9 1]
    \0")),
    Gold => ("gold\0", Lit, Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [.18 .15 .06]
        $envmap editor/cube_vertigo
        $envmapfresnel .6
        $envtintmap [.6 .5 .2]
        $phong 1
        $phongboost 6
        $phongdisablehalflambert 1
        $phongexponent 128
        $phongfresnelranges [.7 .8 1]
        $phongtint [.6 .5 .2]
    \0")),
    Plastic => ("plastic\0", Lit, Some("
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
        $phongfesnelranges [.1 .4 1]
        $phongtint [.8 .9 1]
    \0"))
}
