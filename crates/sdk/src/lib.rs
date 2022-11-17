#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

pub use client::IBaseClientDLL;
pub use client_class::ClientClass;
pub use client_mode::IClientMode;
pub use dt_recv::RecvTable;
pub use engine::IVEngineClient;
pub use entity::PlayerFlag;
pub use global::{set_app, with_app, with_app_mut};
pub use global_vars::{CGlobalVarsBase, Tick, Time};
pub use hit_group::HitGroup;
pub use input::{Button, CInput, CUserCmd};
pub use key_values::KeyValues;
pub use mat4x3::Mat4x3;
pub use net_channel::INetChannel;
pub use ptr::Ptr;
pub use texture_group::TextureGroup;
pub use view_setup::CViewSetup;

pub mod client;
pub mod client_class;
pub mod client_mode;
pub mod console;
pub mod dt_recv;
pub mod engine;
pub mod entity;
pub mod entity_list;
pub mod ffi;
pub mod global_vars;
pub mod hit_group;
pub mod input;
pub mod item_kind;
pub mod key_values;
pub mod mat4x3;
pub mod material;
pub mod model_render;
pub mod net_channel;
pub mod networked;
pub mod ptr;
pub mod texture_group;
pub mod trace;
pub mod view_setup;
pub mod weapon;

pub(crate) mod global;
