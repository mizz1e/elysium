#![feature(addr_parse_ascii)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(decl_macro)]
#![feature(link_llvm_intrinsics)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

pub use client::IBaseClientDLL;
pub use client_class::ClientClass;
pub use client_mode::IClientMode;
pub use color::Color;
pub use config::Config;
pub use convar::{ConVar, ICvar};
pub use dt_recv::{PropKind, RecvTable};
pub use engine::IVEngineClient;
pub use entity::{EntityFlag, IClientEntity, PlayerFlag};
pub use entity_list::IClientEntityList;
pub use error::Error;
pub use gl::{GlContext, GlLoader};
pub use global::{set_app, with_app, with_app_mut};
pub use global_vars::{CGlobalVarsBase, Tick, Time};
pub use hit_group::HitGroup;
pub use input::{Button, CInput, CUserCmd};
pub use key_values::KeyValues;
pub use launcher::Args;
pub use mat4x3::Mat4x3;
pub use material::{IMaterial, IMaterialSystem, MaterialFlag};
pub use model_render::IVModelRender;
pub use module::ModuleMap;
pub use net_channel::INetChannel;
pub use plugin::SourcePlugin;
pub use ptr::Ptr;
pub use sdl::PollEvent;
pub use settings::{OnceLoaded, Renderer, SourceSettings};
pub use texture_group::TextureGroup;
pub use trace::IEngineTrace;
pub use view_setup::CViewSetup;
pub use walking_animation::WalkingAnimation;

pub mod assets;
pub mod client;
pub mod client_class;
pub mod client_mode;
pub mod color;
pub mod config;
pub mod convar;
pub mod dt_recv;
pub mod engine;
pub mod entity;
pub mod entity_list;
pub mod error;
pub mod gl;
pub mod global_vars;
pub mod hit_group;
pub mod iced;
pub mod input;
pub mod intrinsics;
pub mod item_kind;
pub mod key_values;
pub mod launcher;
pub mod mat4x3;
pub mod material;
pub mod math;
pub mod model_render;
pub mod module;
pub mod net_channel;
pub mod networked;
pub mod pattern;
pub mod plugin;
pub mod ptr;
pub mod sdl;
pub mod settings;
pub mod texture_group;
pub mod trace;
pub mod view_setup;
pub mod walking_animation;

pub(crate) mod global;
