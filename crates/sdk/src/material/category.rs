const MAP: phf::Map<&[u8], Category> = phf::phf_map! {
    b"particle\\vistasmokev1\\vistasmokev1_fire.vmt" => Category::Smoke,
    b"particle\\vistasmokev1\\vistasmokev1_emods.vmt" => Category::Smoke,
    b"particle\\vistasmokev1\\vistasmokev1_emods_impactdust.vmt" => Category::Smoke,
    b"particle\\vistasmokev1\\vistasmokev1_smokegrenade.vmt" => Category::Smoke,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Category {
    Decal,
    Smoke,
    World,
}
