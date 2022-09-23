macro_rules! materials {
    ($($variant:ident => ($name:literal, $base:literal, $vdf:expr)),*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum MaterialKind {
            $($variant,)*
        }

        impl MaterialKind {
            /// Material name passed to `Material::new`.
            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(MaterialKind::$variant => $name,)*
                }
            }

            /// Base VDF/KeyValues object passed to the first argument of
            /// [`Vdf::from_bytes`](crate::Vdf::from_bytes).
            #[inline]
            pub const fn base(&self) -> &'static str {
                match self {
                    $(MaterialKind::$variant => $base,)*
                }
            }

            /// VDF/KeyValues passed to the second argument of
            /// [`Vdf::from_bytes`](crate::Vdf::from_bytes).
            #[inline]
            pub const fn vdf(&self) -> Option<&'static str> {
                match self {
                    $(MaterialKind::$variant => $vdf,)*
                }
            }
        }
    };
}

materials! {
    Normal => ("normal", "VertexLitGenric", None),
    Flat => ("flat", "UnlitGeneric", None),
    Chrome => ("chrome", "VertexLitGeneric", Some("
        $envmap env_cubemap
    ")),
    Glow => ("glow", "VertexLitGeneric", Some(r#"
        $additive 1
        $envmap models/effects/cube_white
        $envmapfresnel 1
        $alpha .8
    "#)),
    Pearlescent => ("pearlescent", "VertexLitGeneric", Some("
        $ambientonly 1
        $phong 1
        $pearlescent 3
        $basemapalphaphongmask 1
    ")),
    Metallic => ("metallic", "VertexLitGeneric", Some("
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
    ")),
    Animated => ("animated", "VertexLitGeneric", Some("
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
    ")),
    Platinum => ("platinum", "VertexLitGeneric", Some("
        $basetexture models/player/ct_fbi/ct_fbi_glass
        $envmap env_cubemap
        $envmaptint [.4 .6 .7]
    ")),
    Glass => ("glass", "VertexLitGeneric", Some("
        $additive 1
        $basetexture detail/dt_metal1
        $color [.05 .05 .05]
        $envmap editor/cube_vertigo
    ")),
    Crystal => ("crystal", "VertexLitGeneric", Some("
        $basetexture black
        $bumpmap effects/flat_normal
        $envmap models/effects/crystal_cube_vertigo_hdr
        $envmapfresnel 0
        $phong 1
        $phongboost 2
        $phongexponent 16
        $phongtint [.2 .35 .6]
        $translucent 1
    ")),
    Silver => ("silver", "VertexLitGeneric", Some("
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
    ")),
    Gold => ("gold", "VertexLitGeneric", Some("
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
    ")),
    Plastic => ("plastic", "VertexLitGeneric", Some("
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
    "))
}
