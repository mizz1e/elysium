use crate::config::Pitch;
use crate::{config, global, Color, Config, WalkingAnimation};
use iced_native::{column, row, widget, Command, Element, Length, Program};

const PITCH_LIST: &[Pitch] = &[Pitch::Default, Pitch::Up, Pitch::Down];
const WALKING_ANIMATION_LIST: &[WalkingAnimation] =
    &[WalkingAnimation::Enabled, WalkingAnimation::Disabled];

pub struct Menu;

#[derive(Clone, Debug)]
pub enum Message {
    None,
    Desync(bool),
    DesyncDelta(i32),
    Pitch(Pitch),
    Roll(i32),
    YawOffset(i32),
    WalkingAnimation(WalkingAnimation),
    AntiAimTab,
    VisualsTab,
    Thirdperson(bool),
    ChamColor(String),
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
    global::with_app_mut(move |app| {
        let message = message;
        let mut config = app.world.resource_mut::<Config>();

        match message {
            Message::Desync(enabled) => config.desync_enabled = enabled,
            Message::DesyncDelta(delta) => config.desync_delta = delta as f32,
            Message::Pitch(pitch) => config.pitch = pitch,
            Message::YawOffset(offset) => config.yaw_offset = offset as f32,
            Message::Roll(roll) => config.roll = roll as f32,
            Message::WalkingAnimation(animation) => config.walking_animation = animation,
            Message::AntiAimTab => config.active_tab = 0,
            Message::VisualsTab => config.active_tab = 1,
            Message::Thirdperson(enabled) => config.in_thirdperson = enabled,
            Message::ChamColor(color) => config.cham_color = Color::from_hex_str(&color),
            Message::Load => *config = config::load(),
            Message::Save => config::save(&config),
            Message::None => {}
        }

        Command::none()
    })
}

unsafe fn view_anti_aim<'a>(config: &Config) -> Element<'a, Message, iced_glow::Renderer> {
    let desync_checkbox = widget::checkbox("desync", config.desync_enabled, Message::Desync);

    // debug desync
    // let desync_delta = config.desync_delta.trunc() as i32;
    // let desync_delta_slider = row![
    //     widget::text(format!("desync_delta ({desync_delta}) ")),
    //     widget::slider(-180..=180, desync_delta, Message::DesyncDelta),
    // ];

    let pitch_list = row![
        widget::text("pitch "),
        widget::pick_list(PITCH_LIST, Some(config.pitch), Message::Pitch),
    ];

    let yaw_offset = config.yaw_offset.trunc() as i32;
    let yaw_offset_slider = row![
        widget::text(format!("yaw offset ({yaw_offset}) ")),
        widget::slider(-180..=180, yaw_offset, Message::YawOffset),
    ];

    let roll = config.roll.trunc() as i32;
    let roll_slider = row![
        widget::text(format!("roll ({roll}) ")),
        widget::slider(-50..=50, roll, Message::Roll),
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
    let buttons = row![load_button, save_button].spacing(15);

    let options = column![
        desync_checkbox,
        //desync_delta_slider,
        pitch_list,
        yaw_offset_slider,
        roll_slider,
        walking_animation_list,
        thirdperson_checkbox,
        buttons,
    ];

    let content = widget::scrollable(options.spacing(15));

    content.into()
}

unsafe fn view_visuals<'a>(config: &Config) -> Element<'a, Message, iced_glow::Renderer> {
    let color = config.cham_color.to_hex_string();
    let cham_color_input = widget::text_input("cham color", &color, Message::ChamColor);

    cham_color_input.into()
}

unsafe fn view<'a>() -> Element<'a, Message, iced_glow::Renderer> {
    global::with_app(|app| {
        let config = app.world.resource::<Config>();

        let aa_button = widget::button("anti aim").on_press(Message::AntiAimTab);
        let visuals_button = widget::button("visuals").on_press(Message::VisualsTab);
        let tab_bar = row![aa_button, visuals_button].spacing(15);
        let content = match config.active_tab {
            0 => view_anti_aim(config),
            1 => view_visuals(config),
            _ => unreachable!(),
        };

        let content = column![tab_bar, content,];

        let content = widget::container(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .style(style::custom(style::menu));

        let overlay = widget::container(content)
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

    pub fn custom(f: fn(&Theme) -> container::Appearance) -> theme::Container {
        theme::Container::Custom(Box::from(f))
    }

    pub fn menu(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.7))
    }

    pub fn overlay(_theme: &Theme) -> container::Appearance {
        background(color!(0x000000, 0.2))
    }

    pub fn background(color: Color) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(color)),
            ..container::Appearance::default()
        }
    }
}
