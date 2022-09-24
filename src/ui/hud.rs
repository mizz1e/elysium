use iced_glow::Renderer;
use iced_native::widget::{container, text};
use iced_native::{widget, Alignment, Command, Element, Length, Program};

pub struct Hud;

#[derive(Clone, Debug)]
pub enum Message {
    None,
}

impl Hud {
    pub fn new() -> Self {
        Self
    }
}

impl Program for Hud {
    type Renderer = Renderer;
    type Message = Message;

    #[inline]
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        view(self).into()
    }
}

fn view<'a, Message, Renderer>(hud: &Hud) -> widget::Container<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::Renderer + iced_native::text::Renderer + 'a,
    Renderer::Theme: container::StyleSheet + text::StyleSheet,
{
    let top_left = widget::text("top left");
    let top_centre = widget::text("top centre");
    let top_right = widget::text("top right");

    let centre_left = widget::text("centre left");
    let centre_centre = widget::text("centre centre");
    let centre_right = widget::text("centre right");

    let bottom_left = widget::text("bottom left");
    let bottom_centre = widget::text("bottom centre");
    let bottom_right = widget::text("bottom right");

    let left = column(top_left, centre_left, bottom_left);
    let centre = column(top_centre, centre_centre, bottom_centre);
    let right = column(top_right, centre_right, bottom_right);

    let content = iced_native::row![left, centre, right]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill);

    let ui = widget::container(content)
        .center_x()
        .center_y()
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill);

    ui
}

fn column<'a, Message, Renderer>(
    top: impl Into<Element<'a, Message, Renderer>>,
    centre: impl Into<Element<'a, Message, Renderer>>,
    bottom: impl Into<Element<'a, Message, Renderer>>,
) -> widget::Column<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::Renderer + iced_native::text::Renderer + 'a,
    Renderer::Theme: container::StyleSheet + text::StyleSheet,
{
    let top = widget::container(top)
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill);

    let centre = widget::container(centre)
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill);

    let bottom = widget::container(bottom)
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill);

    let column = iced_native::column![top, centre, bottom]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill);

    column
}
