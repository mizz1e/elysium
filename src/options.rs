use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Options {
    /// Specify an address to connect to immediately (+connect <address>).
    #[clap(long)]
    pub address: Option<SocketAddr>,

    /// Start fullscreen (-fullscreen).
    #[clap(long)]
    pub fullscreen: bool,

    /// Specify map to load immediately (+map <map>).
    #[clap(long)]
    pub map: Option<PathBuf>,

    /// Maximum FPS (+fps_max <fps>).
    #[clap(default_value = "120", long)]
    pub max_fps: u16,

    /// Disable VAC (-insecure).
    #[clap(long)]
    pub no_vac: bool,

    /// Skip the launch video (-novid).
    #[clap(long)]
    pub skip_launch_video: bool,

    /// Use vulkan for rendering.
    #[clap(long)]
    pub vulkan: bool,

    /// Start windowed (-windowed).
    #[clap(long)]
    pub windowed: bool,
}

impl Options {
    #[inline]
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
