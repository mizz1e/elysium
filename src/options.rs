use clap::Parser;
use std::net::SocketAddrV4;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Options {
    /// Specify an address to connect to immediately (+connect \<address\>).
    #[clap(long)]
    pub address: Option<SocketAddrV4>,

    /// Start fullscreen (-fullscreen).
    #[clap(long)]
    pub fullscreen: bool,

    /// Specify map to load immediately (+map \<map\>).
    #[clap(long)]
    pub map: Option<PathBuf>,

    /// Maximum FPS (+fps_max \<fps\>).
    #[clap(long)]
    pub max_fps: Option<u16>,

    /// Disable VAC (-insecure).
    #[clap(long)]
    pub no_vac: bool,

    /// Use vulkan for rendering (ELYSIUM WILL NOT RENDER WITH THIS).
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
