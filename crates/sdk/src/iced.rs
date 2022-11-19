use crate::{gl, global, sdl};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use iced_glow::glow;
use iced_glow::glow::HasContext;
use std::ptr;

pub use menu::Menu;
pub use program::IcedProgram;

mod menu;
mod program;

pub fn render() {
    unsafe {
        global::with_app_mut(|app| {
            let context = app.world.resource::<gl::GlContext>();
            let context = &context.0;
            let viewport = app.world.resource::<sdl::WindowViewport>();
            let viewport = viewport.0.clone();

            if !app.world.contains_resource::<IcedProgram<Menu>>() {
                let program = IcedProgram::from_context(context, viewport, Menu);

                app.insert_resource(program);
            }
        });

        global::with_app_mut(|app| {
            let mut system_state: SystemState<(
                Res<gl::GlContext>,
                Res<sdl::WindowViewport>,
                Option<Res<sdl::CursorPosition>>,
                ResMut<IcedProgram<Menu>>,
            )> = SystemState::new(&mut app.world);

            let (context, viewport, cursor_position, mut program) =
                system_state.get_mut(&mut app.world);

            let context = &context.0;
            let viewport = &viewport.0;
            let cursor_position = cursor_position
                .map(|position| position.0)
                .unwrap_or_default();

            // enable auto-conversion from/to sRGB
            context.enable(glow::FRAMEBUFFER_SRGB);

            // enable alpha blending to not break our fonts
            context.enable(glow::BLEND);
            context.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            context.viewport(
                0,
                0,
                viewport.physical_width() as i32,
                viewport.physical_height() as i32,
            );

            program.update(viewport.clone(), cursor_position);
            program.render(context, viewport.clone());

            // disable auto-conversion from/to sRGB
            context.disable(glow::FRAMEBUFFER_SRGB);

            // disable alpha blending to not break vgui fonts
            context.disable(glow::BLEND);
            context.blend_func(glow::ONE, glow::ZERO);
        });
    }
}
