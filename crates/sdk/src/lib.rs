#![allow(warnings)] // please, shut up.
#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(link_llvm_intrinsics)]
#![feature(pointer_byte_offsets)]
#![feature(exposed_provenance)]
#![feature(strict_provenance)]
#![feature(tuple_trait)]

pub use client::IBaseClientDLL;
pub use client_mode::IClientMode;
pub use client_state::ClientState;
pub use color::Color;
pub use config::Config;
pub use convar::{ConVar, ICvar};
pub use engine::IVEngineClient;
pub use engine_vgui::EngineVGui;
pub use entity::{EntityFlag, IClientEntity, PlayerFlag, WeaponInfo};
pub use entity_list::IClientEntityList;
pub use error::Error;
pub use global::{with_app, with_app_mut};
pub use global_vars::{CGlobalVarsBase, Tick, Time};
pub use hit_group::HitGroup;
pub use input::{Button, CInput, CUserCmd};
pub use input_stack_system::InputStackSystem;
pub use key_values::KeyValues;
pub use mat4x3::Mat4x3;
pub use material::{IMaterial, IMaterialSystem, MaterialFlag};
pub use model_render::IVModelRender;
pub use module::ModuleMap;
pub use net_channel::INetChannel;
pub use physics::{IPhysicsSurfaceProps, SurfaceKind};
pub use plugin::init;
pub use ptr::Ptr;
pub use settings::{OnceLoaded, Renderer, SourceSettings, WindowMode};
pub use surface::Surface;
pub use texture_group::TextureGroup;
pub use trace::{IEngineTrace, TraceResult};
pub use view_setup::CViewSetup;
pub use walking_animation::WalkingAnimation;

pub type Result<T> = std::result::Result<T, Error>;

pub mod assets;
pub mod client;
pub mod client_mode;
pub mod client_state;
pub mod color;
pub mod config;
pub mod convar;
pub mod engine;
pub mod engine_vgui;
pub mod entity;
pub mod entity_list;
pub mod error;
pub mod event;
pub mod global_vars;
pub mod hit_group;
pub mod input;
pub mod input_stack_system;
pub mod intrinsics;
pub mod item_kind;
pub mod key_values;
pub mod mat4x3;
pub mod material;
pub mod math;
pub mod model_render;
pub mod module;
pub mod net_channel;
pub mod net_message;
pub mod networked;
pub mod physics;
pub mod plugin;
pub mod ptr;
pub mod settings;
pub mod surface;
pub mod texture_group;
pub mod trace;
pub mod view_setup;
pub mod walking_animation;

pub(crate) mod global;
