use iced_native::widget::tree::Tree;
use iced_native::widget::{container, tree};
use iced_native::widget::{Container, Widget};
use iced_native::{event, layout, mouse, renderer};
use iced_native::{Clipboard, Event, Layout, Point, Rectangle, Shell, Size};

/// An overlay.
pub struct Overlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    pub overlay: &'a mut Container<'a, Message, Renderer>,
    pub state: &'a mut Tree,
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for Overlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        self.overlay.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.overlay.diff(tree)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();

        self.overlay.draw(
            self.state,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            &bounds,
        );
    }

    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);

        self.overlay.layout(renderer, &limits)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.overlay
            .mouse_interaction(self.state, layout, cursor_position, viewport, renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.overlay.on_event(
            self.state,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn tag(&self) -> tree::Tag {
        self.overlay.tag()
    }

    fn state(&self) -> tree::State {
        self.overlay.state()
    }
}
