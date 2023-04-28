use {
    bevy_source_internal::FnPtr,
    glutin::display::{self, DisplayApiPreference},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    sdl2::{sys, video, VideoSubsystem},
    std::{ffi, ptr},
};

type PollEvent = unsafe extern "C" fn(*mut sys::SDL_Event) -> ffi::c_int;

/// A resource containing window-related data.
pub struct WindowContext {
    /// SDL video subsystem.
    pub video_subsystem: VideoSubsystem,
}

impl WindowContext {
    pub fn new() -> Result<Self, &'static str> {
        let sdl = sdl2::init().map_err(|_| "failed to init sdl")?;
        let video_subsystem = sdl.video().map_err(|_| "failed to init video subsystem")?;

        let result = (sys::SDL_PollEvent as PollEvent).disassemble();

        dbg!(result);

        Ok(Self { video_subsystem })
    }

    pub unsafe fn set_window(&mut self, window: *mut sys::SDL_Window) {
        let window = video::Window::from_ll(self.video_subsystem.clone(), window, ptr::null_mut());
        let display = display::Display::new(window.raw_display_handle(), DisplayApiPreference::Egl);

        println!("{display:?}");
    }
}
