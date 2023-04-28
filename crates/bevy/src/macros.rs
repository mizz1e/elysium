use {
    crate::internal::Library,
    bevy::prelude::*,
    std::{env::consts, io, path::Path},
};

#[inline(never)]
pub fn load_module(ident: &'static str) -> Library {
    const DIRS: [&str; 2] = ["./bin/linux64", "./csgo/bin/linux64"];

    let iter = DIRS
        .iter()
        .map(Path::new)
        .map(|path| path.join(format!("{ident}_client{}", consts::DLL_SUFFIX)))
        .map(|path| unsafe { Library::open(path) });

    let mut error = io::Error::new(io::ErrorKind::NotFound, "unable to find library");

    for result in iter {
        match result {
            Ok(module) => {
                info!("loaded source module `{ident}`");

                return module;
            }
            Err(new_error) if error.kind() != io::ErrorKind::NotFound => {
                error = new_error;
            }
            _ => {}
        }
    }

    error!("failed to load source module `{ident}`: {error}");
    panic!("{error}");
}

macro_rules! modules {
    (
        $($([$meta:meta])*
        pub struct $newtype:ident = $ident:literal;)*
    ) => {
        $(
            $([$meta])*
            #[derive(::bevy::prelude::Resource)]
            pub struct $newtype {
                _module: $crate::internal::Library,
            }

            impl $crate::Module for $newtype {
                const IDENT: &'static str = $ident;

                #[inline]
                fn from_world(_world: &mut ::bevy::prelude::World) -> Self {
                    Self { _module: $crate::macros::load_module(Self::IDENT) }
                }
            }

            impl ::core::ops::Deref for $newtype {
                type Target = $crate::internal::Library;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self._module
                }
            }

            unsafe impl Send for $newtype {}
            unsafe impl Sync for $newtype {}
        )*
    };
}

macro_rules! interfaces {
    (
        $($([$meta:meta])*
        pub struct $newtype:ident = $ident:literal;)*
    ) => {
        $(
            $([$meta])*
            #[derive(::bevy::prelude::Resource)]
            pub struct $newtype {
                _interface: *const u8,
            }

            impl $crate::Interface for $newtype {
                const IDENT: &'static str = $ident;

                #[inline]
                fn from_world(_world: &mut ::bevy::prelude::World) -> Self {
                    Self { _interface: ::core::ptr::null() }
                }
            }

            unsafe impl Send for $newtype {}
            unsafe impl Sync for $newtype {}
        )*
    };
}

pub(crate) use {interfaces, modules};
