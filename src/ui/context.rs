use super::{Hud, Ui};
use crate::assets;
use iced_glow::{glow, Backend, Renderer, Settings, Theme, Viewport};
use iced_native::clipboard::Null;
use iced_native::program::State;
use iced_native::{clipboard, renderer, Color, Debug, Event, Point};

pub struct Context {
    pub clipboard: Null,
    pub hud: State<Hud>,
    pub hud_debug: Debug,
    pub hud_renderer: Renderer,
    pub ui: State<Ui>,
    pub ui_debug: Debug,
    pub ui_renderer: Renderer,
}

impl Context {
    #[inline]
    pub fn new(context: &glow::Context, viewport: Viewport) -> Self {
        let mut hud_renderer = Renderer::new(Backend::new(
            context,
            Settings {
                default_font: Some(assets::QUICKSAND_REGULAR),
                ..Settings::default()
            },
        ));

        let mut ui_renderer = Renderer::new(Backend::new(
            context,
            Settings {
                default_font: Some(assets::QUICKSAND_REGULAR),
                ..Settings::default()
            },
        ));

        let clipboard = clipboard::Null;
        let mut hud_debug = Debug::new();
        let mut ui_debug = Debug::new();

        hud_debug.toggle();

        let hud = Hud::new();
        let ui = Ui::new();

        let hud = State::new(
            hud,
            viewport.logical_size(),
            &mut hud_renderer,
            &mut hud_debug,
        );

        let ui = State::new(ui, viewport.logical_size(), &mut ui_renderer, &mut ui_debug);

        Self {
            clipboard,
            hud,
            hud_debug,
            hud_renderer,
            ui,
            ui_debug,
            ui_renderer,
        }
    }

    #[inline]
    pub fn draw(&mut self, context: &glow::Context, viewport: Viewport) {
        let hud_debug = &mut self.hud_debug;
        let ui_debug = &mut self.ui_debug;
        let hud_renderer = &mut self.hud_renderer;
        let ui_renderer = &mut self.ui_renderer;

        hud_debug.render_started();

        hud_renderer.with_primitives(|backend, primitives| {
            backend.present(&context, primitives, &viewport, &["elysium"]);
        });

        hud_debug.render_finished();

        let state = crate::State::get();

        if state.menu_open.0 {
            ui_renderer.with_primitives(|backend, primitives| {
                backend.present(&context, primitives, &viewport, &ui_debug.overlay());
            });
        }
    }

    #[inline]
    pub fn update(&mut self, viewport: Viewport, cursor_position: Point) {
        let theme = Theme::Dark;
        let style = renderer::Style {
            text_color: Color::WHITE,
        };

        self.hud.update(
            viewport.logical_size(),
            cursor_position,
            &mut self.hud_renderer,
            &theme,
            &style,
            &mut self.clipboard,
            &mut self.hud_debug,
        );

        let state = crate::State::get();

        if state.menu_open.0 {
            self.ui.update(
                viewport.logical_size(),
                cursor_position,
                &mut self.ui_renderer,
                &theme,
                &style,
                &mut self.clipboard,
                &mut self.ui_debug,
            );
        }
    }

    #[inline]
    pub fn queue_event(&mut self, event: Event) {
        self.hud.queue_event(event.clone());
        self.ui.queue_event(event);
    }
}
