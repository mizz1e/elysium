use super::{Element, Layer, Message};
use crate::{
    config,
    config::{AntiAim, Pitch},
    global, Color, Config, IVEngineClient, WalkingAnimation,
};
use bevy::{ecs::system::SystemState, prelude::*};
use iced_native::{column, row, widget, Command, Length, Program};
use std::mem;

const PITCH_LIST: &[Pitch] = &[Pitch::Default, Pitch::Up, Pitch::Down];
const WALKING_ANIMATION_LIST: &[WalkingAnimation] =
    &[WalkingAnimation::Enabled, WalkingAnimation::Disabled];

pub struct Menu;

impl Layer for Menu {
    fn update(&mut self, message: Message) -> Command<Message> {
        unsafe { update(message) }
    }

    fn view(&self) -> Element<'_> {
        unsafe { view() }
    }
}

fn update(message: Message) -> Command<Message> {
    global::with_app_mut(move |app| {
        let message = message;

        let mut system_state: SystemState<(ResMut<Config>, Res<IVEngineClient>)> =
            SystemState::new(&mut app.world);

        let (mut config, engine) = system_state.get_mut(&mut app.world);

        if !config.menu_open {
            return Command::none();
        }

        match message {
            Message::AutoShoot(enabled) => config.auto_shoot = enabled,

            Message::AntiAim(enabled) => config.anti_aim.enabled = enabled,

            Message::Pitch(pitch) => config.anti_aim.pitch = pitch,
            Message::YawOffset(offset) => config.anti_aim.yaw_offset = offset,
            Message::Roll(roll) => config.anti_aim.roll = roll,

            Message::FakePitch(pitch) => config.anti_aim.fake_pitch = pitch,
            Message::FakeYawOffset(offset) => config.anti_aim.fake_yaw_offset = offset,
            Message::FakeRoll(roll) => config.anti_aim.fake_roll = roll,

            Message::FakeLag(value) => config.fake_lag = value,

            Message::WalkingAnimation(animation) => config.walking_animation = animation,
            Message::AntiAimTab => config.active_tab = 0,
            Message::RageBotTab => config.active_tab = 1,
            Message::VisualsTab => config.active_tab = 2,
            Message::Thirdperson(enabled) => config.in_thirdperson = enabled,
            Message::ChamColor(color) => config.cham_color = Color::from_hex_str(&color),
            Message::Load => *config = config::load(),
            Message::Save => config::save(&config),
            Message::None => {}

            Message::Command(command) => config.command = command,
            Message::RunCommand => {
                engine.run_command(&mem::take(&mut config.command));
            }
        }

        Command::none()
    })
}

fn view_modifiers<'a>(
    label: &'static str,
    pitch: Pitch,
    yaw_offset: f32,
    roll: f32,
    on_change_pitch: impl Fn(Pitch) -> Message + 'a,
    on_change_yaw_offset: impl Fn(f32) -> Message + 'a,
    on_change_roll: impl Fn(f32) -> Message + 'a,
) -> Element<'a> {
    let label = widget::text(label);

    let pitch_list = row![
        widget::text("pitch "),
        widget::pick_list(PITCH_LIST, Some(pitch), on_change_pitch),
    ];

    let yaw_offset = yaw_offset.trunc() as i32;
    let yaw_offset_slider = row![
        widget::text(format!("yaw offset ({yaw_offset}) ")),
        widget::slider(-180..=180, yaw_offset, move |offset| on_change_yaw_offset(
            offset as f32
        )),
    ];

    let roll = roll.trunc() as i32;
    let roll_slider = row![
        widget::text(format!("roll ({roll}) ")),
        widget::slider(-50..=50, roll, move |value| on_change_roll(value as f32)),
    ];

    column![label, pitch_list, yaw_offset_slider, roll_slider,].into()
}

fn view_anti_aim<'a>(config: &Config) -> Element<'a> {
    let aa = &config.anti_aim;
    let enabled_checkbox = widget::checkbox("enabled", aa.enabled, Message::AntiAim);

    let real = view_modifiers(
        "real",
        aa.pitch,
        aa.yaw_offset,
        aa.roll,
        Message::Pitch,
        Message::YawOffset,
        Message::Roll,
    );

    let fake = view_modifiers(
        "fake",
        aa.fake_pitch,
        aa.fake_yaw_offset,
        aa.fake_roll,
        Message::FakePitch,
        Message::FakeYawOffset,
        Message::FakeRoll,
    );

    let fake_lag = config.fake_lag;
    let fake_lag_slider = row![
        widget::text(format!("fake lag ({fake_lag}) ")),
        widget::slider(0..=14, fake_lag, Message::FakeLag),
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

    let command_input = widget::text_input("command", &config.command, Message::Command);
    let run_command_button = widget::button("run command").on_press(Message::RunCommand);

    let options = column![
        enabled_checkbox,
        real,
        fake,
        fake_lag_slider,
        walking_animation_list,
        thirdperson_checkbox,
        buttons,
        command_input,
        run_command_button,
    ];

    let content = widget::scrollable(options.spacing(15));

    content.into()
}

fn view_rage_bot<'a>(config: &Config) -> Element<'a> {
    let auto_shoot_checkbox = widget::checkbox("auto shoot", config.auto_shoot, Message::AutoShoot);
    let options = column![auto_shoot_checkbox];

    let content = widget::scrollable(options.spacing(15));

    content.into()
}

fn view_visuals<'a>(config: &Config) -> Element<'a> {
    let color = config.cham_color.to_hex_string();
    let cham_color_input = widget::text_input("cham color", &color, Message::ChamColor);

    cham_color_input.into()
}

fn view<'a>() -> Element<'a> {
    global::with_app(|app| {
        let config = app.world.resource::<Config>();

        if !config.menu_open {
            return widget::Space::new(Length::Fill, Length::Fill).into();
        }

        let aa_button = widget::button("anti aim").on_press(Message::AntiAimTab);
        let ragebot_button = widget::button("ragebot").on_press(Message::RageBotTab);
        let visuals_button = widget::button("visuals").on_press(Message::VisualsTab);
        let tab_bar = row![aa_button, ragebot_button, visuals_button].spacing(15);
        let content = match config.active_tab {
            0 => view_anti_aim(config),
            1 => view_rage_bot(config),
            2 => view_visuals(config),
            _ => unreachable!(),
        };

        let content = column![tab_bar, content].spacing(20);
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
