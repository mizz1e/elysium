use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Options {
    /// Specify an address to connect to immediately.
    #[clap(long)]
    pub address: Option<SocketAddr>,

    /// Maximum FPS.
    #[clap(default_value = "120", long)]
    pub fps: u16,

    /// Specify map to load immediately.
    #[clap(long)]
    pub map: Option<PathBuf>,

    /// Disable VAC.
    #[clap(long)]
    pub no_vac: bool,
}

impl Options {
    #[inline]
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }
}
