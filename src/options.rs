use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
pub struct Options {
    /// Specify address to connect to immediately.
    #[clap(long)]
    pub address: Option<SocketAddr>,

    /// Enable cheats.
    #[clap(long)]
    pub i_agree_to_be_banned: bool,

    /// Specify map to load immediately.
    #[clap(long)]
    pub map: Option<String>,

    /// Default FPS.
    #[clap(default_value = "120", long)]
    pub fps: u16,
}

impl Options {
    #[inline]
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }
}
