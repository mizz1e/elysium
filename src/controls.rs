use crate::State;
use iced_glow::Renderer;
use iced_native::theme::Container;
use iced_native::{widget, Command, Element, Length, Program};

#[derive(Default)]
pub struct Controls {}

#[derive(Clone, Debug)]
pub enum Message {
    AntiAim(bool),
    Thirdperson(bool),
    None,
}

impl Controls {
    #[inline]
    pub fn new() -> Controls {
        Default::default()
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    #[inline]
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let state = State::get();

        match message {
            Message::AntiAim(value) => state.local.anti_aim = value,
            Message::Thirdperson(value) => state.local.thirdperson.0 = value,
            _ => {}
        }

        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        let state = State::get();
        let anti_aim = widget::checkbox("Anti-Aim", state.local.anti_aim, Message::AntiAim);

        // this does work, if you have a local player, whilest in the main menu it would seem
        // broken!
        //
        // TODO: rework thirdperson code into user choice, current state, and input lock
        let thirdperson = widget::checkbox(
            "Thirdperson",
            state.local.thirdperson.0,
            Message::Thirdperson,
        );

        let content = iced_native::column![anti_aim, thirdperson].spacing(15);

        let menu = widget::container(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y()
            .style(Container::Custom(style::menu));

        let overlay = widget::container(menu)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Container::Custom(style::overlay));

        overlay.into()
    }
}

mod style {
    use iced_native::widget::container;
    use iced_native::{Background, Color, Theme};

    #[inline]
    pub fn menu(_theme: &Theme) -> container::Appearance {
        background(Color::from_rgba8(0xEF, 0xD9, 0xC3, 1.0))
    }

    #[inline]
    pub fn overlay(_theme: &Theme) -> container::Appearance {
        background(Color::from_rgba8(0, 0, 0, 0.2))
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
