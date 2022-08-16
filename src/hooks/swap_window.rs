use crate::{Menu, State};
use core::mem::MaybeUninit;
use glow::HasContext;
use iced_glow::{glow, Viewport};
use iced_native::Size;
use sdl2_sys::{SDL_GetWindowSize, SDL_Window};

#[inline]
unsafe fn window_size(window: *mut SDL_Window) -> Size<u32> {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();

    SDL_GetWindowSize(window, width.as_mut_ptr(), height.as_mut_ptr());

    let width = width.assume_init();
    let height = height.assume_init();
    let size = Size::new(width as u32, height as u32);

    size
}

/// `SDL_GL_SwapWindow` hook.
pub unsafe extern "C" fn swap_window(window: *mut sdl2_sys::SDL_Window) {
    let state = State::get();
    let swap_window_original = state.hooks.swap_window.unwrap();

    state.window_size = window_size(window);

    let context = state.context.get_or_insert_with(|| {
        let context = glow::Context::from_loader_function(|symbol| {
            let get_proc_address = state.get_proc_address.unwrap_unchecked();

            (get_proc_address)(symbol.as_ptr()) as _
        });

        context
    });

    // enable auto-conversion from/to sRGB
    context.enable(glow::FRAMEBUFFER_SRGB);

    // enable alpha blending to not break our fonts
    context.enable(glow::BLEND);
    context.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

    let viewport = Viewport::with_physical_size(state.window_size, 1.0);
    let menu = state
        .menu
        .get_or_insert_with(|| Menu::new(context, viewport.clone()));

    if state.menu_open.0 {
        context.viewport(
            0,
            0,
            state.window_size.width as i32,
            state.window_size.height as i32,
        );

        menu.update(viewport.clone(), state.cursor_position);
        menu.draw(context, viewport);
    }

    // disable auto-conversion from/to sRGB
    context.disable(glow::FRAMEBUFFER_SRGB);

    // disable alpha blending to not break vgui fonts
    context.disable(glow::BLEND);

    (swap_window_original)(window);
}
