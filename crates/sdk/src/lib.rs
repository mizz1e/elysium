#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

pub use global::{set_app, with_app, with_app_mut};
pub use hit_group::HitGroup;
pub use id::SteamId;
pub use mat4x3::Mat4x3;
pub use ptr::Ptr;
pub use vdf::Vdf;
pub use view::View;

mod client_mode;
mod console;
mod engine;
mod hit_group;
mod item_kind;
mod mat4x3;
mod physics;
mod render;
mod steam;
mod vdf;
mod view;
mod weapon;

pub mod client;
pub mod entity;
pub mod entity_list;
pub mod ffi;
pub mod global_vars;
pub mod id;
pub mod input;
pub mod material;
pub mod model_render;
pub mod network_channel;
pub mod networked;
pub mod player_model;
pub mod ptr;
pub mod trace;

pub(crate) mod global;
