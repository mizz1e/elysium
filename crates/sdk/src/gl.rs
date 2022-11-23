use crate::global;
use bevy::prelude::*;
use iced_glow::glow;
use std::ffi;
use std::ffi::CString;

#[derive(Resource)]
pub struct GlLoader {
    library: libloading::Library,
    loader: unsafe extern "C" fn(symbol: *const ffi::c_char) -> *const ffi::c_void,
}

impl GlLoader {
    pub unsafe fn setup() -> Self {
        tracing::trace!("obtain eglGetProcAddress");

        let library = libloading::Library::new("libEGL.so").unwrap();
        let loader = *library.get(b"eglGetProcAddress\0").unwrap();

        tracing::trace!("eglGetProcAddress = {loader:?}");

        Self { library, loader }
    }
}

#[derive(Deref, Resource)]
pub struct GlContext(pub glow::Context);

pub unsafe fn setup() -> GlContext {
    tracing::trace!("obtain glow::Context");

    global::with_app(|app| {
        let loader = app.world.resource::<GlLoader>();
        let context = glow::Context::from_loader_function(|symbol| {
            let symbol = CString::new(symbol).unwrap();

            (loader.loader)(symbol.as_ptr())
        });

        tracing::trace!("glow::Context = {context:?}");

        GlContext(context)
    })
}
