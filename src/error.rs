use bevy::utils::thiserror;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to find CSGO")]
    NoCsgo,

    #[error("unable to find X11 DISPLAY")]
    NoDisplay,

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
