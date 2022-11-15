use super::{Hud, Ui};
use crate::assets;
use iced_glow::{glow, Backend, Renderer, Settings, Theme, Viewport};
use iced_native::clipboard::Null;
use iced_native::program::State;
use iced_native::{clipboard, renderer, Color, Debug, Event, Point};

pub struct Context {
    pub clipboard: Null,
    pub debug: Debug,
    pub hud: State<Hud>,
    pub hud_renderer: Renderer,
    pub ui: State<Ui>,
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
        let mut debug = Debug::new();
        let hud = Hud::new();
        let ui = Ui::new();

        let hud = State::new(hud, viewport.logical_size(), &mut hud_renderer, &mut debug);
        let ui = State::new(ui, viewport.logical_size(), &mut ui_renderer, &mut debug);

        Self {
            clipboard,
            debug,
            hud,
            hud_renderer,
            ui,
            ui_renderer,
        }
    }

    #[inline]
    pub fn draw(&mut self, context: &glow::Context, viewport: Viewport) {
        const EMPTY: &[&str] = &[];

        let hud_renderer = &mut self.hud_renderer;
        let ui_renderer = &mut self.ui_renderer;
        let state = crate::State::get();

        if let Some(interfaces) = state.interfaces.as_ref() {
            if interfaces.engine.is_in_game() {
                /*hud_renderer.with_primitives(|backend, primitives| {
                    backend.present(&context, primitives, &viewport, EMPTY);
                });*/
            }
        }

        if state.menu_open.0 {
            ui_renderer.with_primitives(|backend, primitives| {
                backend.present(&context, primitives, &viewport, EMPTY);
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
            &mut self.debug,
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
                &mut self.debug,
            );
        }
    }

    #[inline]
    pub fn queue_event(&mut self, event: Event) {
        self.hud.queue_event(event.clone());
        self.ui.queue_event(event);
    }
}
