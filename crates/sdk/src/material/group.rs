// https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/public/texture_group_names.h

macro_rules! group {
    ($($group:ident => $bytes:literal,)*) => {
        const MAP: phf::Map<&[u8], Group> = phf::phf_map! {
            $($bytes => Group::$group,)*
        };

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Group<'a> {
            $($group,)*
            Unknown(&'a [u8]),
        }

        impl<'a> Group<'a> {
            #[inline]
            pub fn as_bytes(&self) -> &'a [u8] {
                match self {
                    $(Group::$group => $bytes,)*
                    Group::Unknown(group) => group,
                }
            }

            #[inline]
            pub fn from_bytes(bytes: &'a [u8]) -> Self {
                MAP.get(bytes)
                    .copied()
                    .unwrap_or_else(|| Group::Unknown(bytes))
            }
        }
    };
}

group! {
    ClientEffect => b"ClientEffect textures",
    Cubemap => b"CubeMap textures",
    Decal => b"Decal textures",
    DepthBuffer => b"DepthBuffer",
    DisplacementVertex => b"Displacement Verts",
    DynamicIndex => b"Dynamic Indices",
    DynamicVertex => b"Dynamic Verts",
    LightingVertex => b"Lighting Verts",
    Model => b"Model textures",
    ModelVertex => b"Model Verts",
    MorphTarget => b"Morph Targets",
    Other => b"Other textures",
    OtherVertex => b"Other Verts",
    Particle => b"Particle textures",
    PixelShader => b"Pixel Shaders",
    Precached => b"Precached",
    PreloadTexture => b"texture preload",
    RenderTarget => b"RenderTargets",
    RenderTargetSurface => b"RenderTarget Surfaces",
    StaticIndex => b"Static Indices",
    StaticProp => b"StaticProp textures",
    StaticVertex => b"Static Vertex",
    Skybox => b"SkyBox textures",
    World => b"World textures",
    WorldVertex => b"World Verts",
    Unaccounted => b"Unaccounted textures",
    VertexShader => b"Vertex Shaders",
    Vgui => b"VGUI textures",
    ViewModel => b"ViewModel",
}
