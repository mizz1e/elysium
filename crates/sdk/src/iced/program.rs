use crate::assets;
use bevy::prelude::Resource;
use iced_glow::{glow, Backend, Renderer, Settings, Viewport};
use iced_native::program::State;
use iced_native::theme::{Palette, Theme};
use iced_native::{clipboard, color, renderer, Color, Debug, Event, Point, Program};
use std::{fmt, mem};

#[derive(Resource)]
pub struct IcedProgram<P: Program + 'static> {
    pub clipboard: clipboard::Null,
    pub debug: Debug,
    pub renderer: Renderer,
    pub state: State<P>,
    pub damaged: bool,
}

impl<P, M> IcedProgram<P>
where
    P: Program<Renderer = Renderer, Message = M> + 'static,
    M: fmt::Debug + Send,
{
    pub fn from_context(context: &glow::Context, viewport: Viewport, program: P) -> Self {
        let clipboard = clipboard::Null;
        let mut debug = Debug::new();

        let settings = Settings {
            default_font: Some(assets::QUICKSAND_REGULAR),
            ..Settings::default()
        };

        let mut renderer = Renderer::new(Backend::new(context, settings));
        let state = State::new(program, viewport.logical_size(), &mut renderer, &mut debug);
        let damaged = true;

        Self {
            clipboard,
            debug,
            renderer,
            state,
            damaged,
        }
    }

    pub fn render(&mut self, context: &glow::Context, viewport: Viewport) {
        const DEBUG_OVERLAY: &[&str] = &[];

        self.renderer.with_primitives(|backend, primitives| {
            backend.present(&context, primitives, &viewport, DEBUG_OVERLAY);
        });
    }

    pub fn update(&mut self, viewport: Viewport, cursor_position: Point) {
        /*if !mem::take(&mut self.damaged) {
            return;
        }*/

        let theme = Theme::custom(Palette {
            primary: color!(0xa00000),
            ..Palette::DARK
        });

        let style = renderer::Style {
            text_color: Color::WHITE,
        };

        self.state.update(
            viewport.logical_size(),
            cursor_position,
            &mut self.renderer,
            &theme,
            &style,
            &mut self.clipboard,
            &mut self.debug,
        );
    }

    pub fn queue_event(&mut self, event: Event) {
        self.damaged = true;
        self.state.queue_event(event);
    }
}

unsafe impl<P: Program + 'static> Send for IcedProgram<P> {}
unsafe impl<P: Program + 'static> Sync for IcedProgram<P> {}
