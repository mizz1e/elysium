#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

pub use beam::{Beam, BeamInfo, ViewRenderBeams};
pub use client::Client;
pub use client_mode::ClientMode;
pub use console::{Console, Var, VarKind, Vars};
pub use engine::{Engine, PlayerInfo};
pub use entity::EntityList;
pub use frame::Frame;
pub use global::{set_app, with_app, with_app_mut};
pub use globals::Globals;
pub use hit_group::HitGroup;
pub use id::SteamId;
pub use input::{Command, Input};
pub use input_system::InputSystem;
pub use item_kind::ItemKind;
pub use mat4x3::Mat4x3;
pub use network::{Flow, NetworkChannel};
pub use ptr::Ptr;
pub use render::{OverrideKind, Render};
pub use steam::SteamAPIContext;
pub use surface::Surface;
pub use vdf::Vdf;
pub use view::View;
pub use weapon::{WeaponInfo, WeaponKind};

mod beam;
mod client_mode;
mod console;
mod engine;
mod frame;
mod global;
mod globals;
mod hit_group;
mod input_system;
mod item_kind;
mod mat4x3;
mod physics;
mod render;
mod steam;
mod surface;
mod vdf;
mod view;
mod weapon;

pub mod client;
pub mod entity;
pub mod ffi;
pub mod id;
pub mod input;
pub mod material;
pub mod model;
pub mod network;
pub mod networked;
pub mod player_model;
pub mod ptr;
pub mod trace;
