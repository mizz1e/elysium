use bevy::prelude::*;
use std::ffi;
use std::net::SocketAddrV4;
use std::path::PathBuf;

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
    /// Returns the argument passed to tier0's command line.

    pub(crate) fn arg(&self) -> &'static str {
        match self {
            Renderer::OpenGl => "-opengl",
            Renderer::Vulkan => "-vulkan",
            Renderer::Headless => "-noshaderapi",
        }
    }

    /// Returns a display string for this renderer.

    pub(crate) fn display(&self) -> &'static str {
        match self {
            Renderer::OpenGl => "DX9 (OpenGL)",
            Renderer::Vulkan => "Vulkan",
            Renderer::Headless => "Headless",
        }
    }

    /// Returns the value returned by *pMaterialSystem + 164 (GetShaderAPI).

    pub(crate) fn value(&self) -> ffi::c_int {
        match self {
            Renderer::OpenGl => 2,
            Renderer::Vulkan => 3,
            Renderer::Headless => 1,
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

/// Source engine settings resource.
#[derive(Debug, Default, Resource)]
pub struct SourceSettings {
    /// The default maximum FPS (`+fps_max <fps>`).
    pub max_fps: Option<u16>,

    /// Actions to perform once the engine is loaded.
    pub once_loaded: OnceLoaded,

    /// The renderer to use.
    pub renderer: Renderer,
}
