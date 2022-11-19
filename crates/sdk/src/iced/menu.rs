use crate::config::Pitch;
use crate::{config, global, Config, WalkingAnimation};
use iced_native::{column, row, widget, Command, Element, Length, Program};

const PITCH_LIST: &[Pitch] = &[Pitch::Default, Pitch::Up, Pitch::Down];
const WALKING_ANIMATION_LIST: &[WalkingAnimation] =
    &[WalkingAnimation::Enabled, WalkingAnimation::Disabled];

pub struct Menu;

#[derive(Clone, Debug)]
pub enum Message {
    Desync(bool),
    Pitch(Pitch),
    YawOffset(i32),
    WalkingAnimation(WalkingAnimation),
    Thirdperson(bool),
    Load,
    Save,
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
            Message::Pitch(pitch) => config.pitch = pitch,
            Message::YawOffset(offset) => config.yaw_offset = offset as f32,
            Message::WalkingAnimation(animation) => config.walking_animation = animation,
            Message::Thirdperson(enabled) => config.in_thirdperson = enabled,
            Message::Load => *config = config::load(),
            Message::Save => config::save(&config),
        }

        Command::none()
    })
}

unsafe fn view<'a>() -> Element<'a, Message, iced_glow::Renderer> {
    global::with_app(|app| {
        let config = app.world.resource::<Config>();

        let desync_checkbox = widget::checkbox("desync", config.desync_enabled, Message::Desync);

        let pitch_list = row![
            widget::text("pitch "),
            widget::pick_list(PITCH_LIST, Some(config.pitch), Message::Pitch),
        ];

        let yaw_offset_slider = row![
            widget::text("yaw offset "),
            widget::slider(
                -180..=180,
                config.yaw_offset.trunc() as i32,
                Message::YawOffset,
            )
        ];

        let walking_animation_list = row![
            widget::text("walking animation "),
            widget::pick_list(
                WALKING_ANIMATION_LIST,
                Some(config.walking_animation),
                Message::WalkingAnimation,
            ),
        ];

        let thirdperson_checkbox =
            widget::checkbox("thirdperson", config.in_thirdperson, Message::Thirdperson);

        let load_button = widget::button("load").on_press(Message::Load);
        let save_button = widget::button("save").on_press(Message::Save);

        let options = column![
            desync_checkbox,
            pitch_list,
            yaw_offset_slider,
            walking_animation_list,
            thirdperson_checkbox,
            load_button,
            save_button
        ]
        .spacing(15);
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
