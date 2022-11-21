macro_rules! group {
    ($($group:ident => $bytes:literal,)*) => {
        /// Texture group.
        ///
        /// Used within `pMaterialSystem->FindMaterial`.
        ///
        /// See [`public/texture_group_names.h`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/texture_group_names.h).
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum TextureGroup {
            $($group,)*
        }

        impl TextureGroup {

            pub fn as_bytes(&self) -> &'static [u8] {
                match self {
                    $(TextureGroup::$group => $bytes,)*
                }
            }


            pub fn from_bytes(bytes: &[u8]) -> Self {
                const MAP: phf::Map<&[u8], TextureGroup> = phf::phf_map! {
                    $($bytes => TextureGroup::$group,)*
                };

                MAP.get(bytes)
                    .copied()
                    .unwrap_or_else(|| panic!("unknown texture group: {:?}", String::from_utf8_lossy(bytes)))
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
    VGui => b"VGUI textures",
    ViewModel => b"ViewModel",
}
