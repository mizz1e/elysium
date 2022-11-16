#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

pub use client::IBaseClientDLL;
pub use client_mode::IClientMode;
pub use engine::IVEngineClient;
pub use global::{set_app, with_app, with_app_mut};
pub use global_vars::{CGlobalVarsBase, Tick, Time};
pub use hit_group::HitGroup;
pub use id::SteamId;
pub use input::{Button, CInput, CUserCmd};
pub use mat4x3::Mat4x3;
pub use net_channel::INetChannel;
pub use ptr::Ptr;
pub use texture_group::TextureGroup;
pub use vdf::Vdf;
pub use view_setup::CViewSetup;

mod client;
mod client_mode;
mod console;
mod engine;
mod global_vars;
mod hit_group;
mod input;
mod item_kind;
mod mat4x3;
mod net_channel;
mod physics;
mod render;
mod steam;
mod texture_group;
mod vdf;
mod view_setup;
mod weapon;

pub mod entity;
pub mod entity_list;
pub mod ffi;
pub mod id;
pub mod material;
pub mod model_render;
pub mod networked;
pub mod player_model;
pub mod ptr;
pub mod trace;

pub(crate) mod global;
