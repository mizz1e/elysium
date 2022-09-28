// https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/public/texture_group_names.h

const MAP: phf::Map<&[u8], Group> = phf::phf_map! {
    b"ClientEffect textures" => Group::ClientEffect,
    b"CubeMap textures" => Group::Cubemap,
    b"Decal textures" => Group::Decal,
    b"DepthBuffer" => Group::DepthBuffer,
    b"Displacement Verts" => Group::DisplacementVertex,
    b"Dynamic Indices" => Group::DynamicIndex,
    b"Dynamic Verts" => Group::DynamicVertex,
    b"Lighting Verts" => Group::LightingVertex,
    b"Model textures" => Group::Model,
    b"Model Verts" => Group::ModelVertex,
    b"Morph Targets" => Group::MorphTarget,
    b"Other textures" => Group::Other,
    b"Other Verts" => Group::OtherVertex,
    b"Particle textures" => Group::Particle,
    b"Pixel Shaders" => Group::PixelShader,
    b"Precached" => Group::Precached,
    b"RenderTargets" => Group::RenderTarget,
    b"RenderTarget Surfaces" => Group::RenderTargetSurface,
    b"Static Indices" => Group::StaticIndex,
    b"Static Vertex" => Group::StaticVertex,
    b"SkyBox textures" => Group::Skybox,
    b"World textures" => Group::World,
    b"World Verts" => Group::WorldVertex,
    b"Unaccounted textures" => Group::Unaccounted,
    b"Vertex Shaders" => Group::VertexShader,
    b"VGUI textures" => Group::Vgui,
    b"ViewModel" => Group::ViewModel,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Group<'a> {
    ClientEffect,
    Cubemap,
    Decal,
    DepthBuffer,
    DisplacementVertex,
    DynamicIndex,
    DynamicVertex,
    LightingVertex,
    Model,
    ModelVertex,
    MorphTarget,
    Other,
    OtherVertex,
    Particle,
    PixelShader,
    Precached,
    RenderTarget,
    RenderTargetSurface,
    StaticIndex,
    StaticVertex,
    Skybox,
    World,
    WorldVertex,
    Unaccounted,
    VertexShader,
    Vgui,
    ViewModel,

    Unknown(&'a [u8]),
}

impl<'a> Group<'a> {
    #[inline]
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        MAP.get(bytes)
            .copied()
            .unwrap_or_else(|| Group::Unknown(bytes))
    }
}
