use bevy::prelude::Resource;
use std::ffi;
use std::net::SocketAddrV4;
use std::path::PathBuf;

pub struct RendererParams {
    pub display: &'static str,
    pub option: &'static str,
    pub value: ffi::c_int,
}

/// The renderer backend to use (Shader API).
#[derive(Debug, Default)]
pub enum Renderer {
    /// DX9 (OpenGL).
    ///
    /// Implemented within `shaderapidx9_client.so`.
    #[default]
    OpenGl,

    /// Vulkan.
    ///
    /// Implemented within `shaderapivk_client.so`.
    Vulkan,

    /// Headless.
    ///
    /// Implemented within `shaderapiempty_client.so`. (May not be present in a regular install of
    /// CSGO).
    Headless,
}

impl Renderer {
    pub(crate) fn params(&self) -> RendererParams {
        match self {
            Renderer::OpenGl => RendererParams {
                display: "DX9 (OpenGL)",
                option: "-opengl",
                value: 2,
            },
            Renderer::Vulkan => RendererParams {
                display: "Vulkan",
                option: "-vulkan",
                value: 3,
            },
            Renderer::Headless => RendererParams {
                display: "Headless",
                option: "-noshaderapi",
                value: 1,
            },
        }
    }
}

/// Actions to perform once the engine is loaded.
#[derive(Debug, Default)]
pub enum OnceLoaded {
    #[default]
    Nothing,

    /// An address to connect to.
    ConnectTo(SocketAddrV4),

    /// A map to load.
    LoadMap(PathBuf),
}

/// Defines the way the window is displayed.
#[derive(Debug, Default)]
pub enum WindowMode {
    #[default]
    Last,
    Windowed,
    Fullscreen,
}

/// Source engine settings resource.
#[derive(Debug, Default, Resource)]
pub struct SourceSettings {
    /// The default maximum FPS (`+fps_max <fps>`).
    pub max_fps: Option<u16>,

    /// Disable Valve Anti-Cheat.
    pub no_vac: bool,

    /// Actions to perform once the engine is loaded.
    pub once_loaded: OnceLoaded,

    /// The renderer to use.
    pub renderer: Renderer,

    /// Defines the way the window is displayed.
    pub window_mode: WindowMode,
}
