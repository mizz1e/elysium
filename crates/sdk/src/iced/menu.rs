use crate::{global, Config};
use iced_native::{column, row, widget, Command, Element, Length, Program};

pub struct Menu;

#[derive(Clone, Debug)]
pub enum Message {
    Desync(bool),
    YawOffset(i32),
}

impl Program for Menu {
    type Renderer = iced_glow::Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        unsafe { update(message) }
    }

    fn view(&self) -> Element<'_, Message, iced_glow::Renderer> {
        unsafe { view() }
    }
}

unsafe fn update(message: Message) -> Command<Message> {
    global::with_app_mut(|app| {
        let mut config = app.world.resource_mut::<Config>();

        match message {
            Message::Desync(enabled) => config.desync_enabled = enabled,
            Message::YawOffset(offset) => config.yaw_offset = offset as f32,
        }

        Command::none()
    })
}

unsafe fn view<'a>() -> Element<'a, Message, iced_glow::Renderer> {
    global::with_app(|app| {
        let config = app.world.resource::<Config>();

        let desync_checkbox = widget::checkbox("desync", config.desync_enabled, Message::Desync);
        let yaw_offset_slider = row![
            widget::text("yaw offset "),
            widget::slider(
                -180..=180,
                config.yaw_offset.trunc() as i32,
                Message::YawOffset,
            )
        ];

        let options = column![desync_checkbox, yaw_offset_slider].spacing(15);
        let content = widget::scrollable(options);

        let menu = widget::container(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y()
            .padding(20)
            .style(style::custom(style::menu));

        let overlay = widget::container(menu)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::custom(style::overlay));

        overlay.into()
    })
}

mod style {
    use iced_native::widget::container;
    use iced_native::{color, theme, Background, Color, Theme};

    #[inline]
    pub fn custom(f: fn(&Theme) -> container::Appearance) -> theme::Container {
        theme::Container::Custom(Box::from(f))
    }

    #[inline]
    pub fn menu(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.7))
    }

    #[inline]
    pub fn overlay(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.2))
    }

    #[inline]
    pub fn background(color: Color) -> container::Appearance {
        let appearance = container::Appearance {
            background: Some(Background::Color(color)),
            ..container::Appearance::default()
        };

        appearance
    }
}
