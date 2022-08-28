use crate::State;
use core::ops::RangeInclusive;
use iced_glow::Renderer;
use iced_native::theme::Container;
use iced_native::{widget, Command, Element, Length, Program};

#[derive(Default)]
pub struct Controls {}

#[derive(Clone, Debug)]
pub enum Message {
    AntiAim(bool),
    Thirdperson(bool),

    FogRed(f32),
    FogGreen(f32),
    FogBlue(f32),
    FogAlpha(f32),

    FogStart(f32),
    FogEnd(f32),
    FogClip(f32),

    Bloom(f32),

    ExposureMin(f32),
    ExposureMax(f32),

    FakeLag(u8),

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

            Message::FogRed(value) => state.fog.color.red = value,
            Message::FogGreen(value) => state.fog.color.green = value,
            Message::FogBlue(value) => state.fog.color.blue = value,
            Message::FogAlpha(value) => state.fog.alpha = value,

            Message::FogStart(value) => state.fog_start = value,
            Message::FogEnd(value) => state.fog_end = value,
            Message::FogClip(value) => state.fog_clip = value,

            Message::Bloom(value) => state.bloom = value,

            Message::ExposureMin(value) => state.exposure_min = value,
            Message::ExposureMax(value) => state.exposure_max = value,

            Message::FakeLag(value) => state.fake_lag = value,

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

        const COMPONENT_RANGE: RangeInclusive<f32> = 0.0..=1.0;
        const FOG_RANGE: RangeInclusive<f32> = 0.0..=10_000.0;
        const BLOOM_RANGE: RangeInclusive<f32> = 0.0..=5.0;
        const EXPOSURE_RANGE: RangeInclusive<f32> = 0.0..=10.0;

        // TODO: cl move client-side cap fix
        // TODO: check sv_maxusrcmdprocessticks
        const FAKE_LAG_RANGE: RangeInclusive<u8> = 0..=16;

        let fake_lag = iced_native::row![
            widget::text("Fake Lag"),
            widget::slider(FAKE_LAG_RANGE, state.fake_lag, Message::FakeLag),
        ];

        let red = iced_native::row![
            widget::text("Fog red"),
            widget::slider(COMPONENT_RANGE, state.fog.color.red, Message::FogRed).step(0.01),
        ];

        let green = iced_native::row![
            widget::text("Fog green"),
            widget::slider(COMPONENT_RANGE, state.fog.color.green, Message::FogGreen).step(0.01),
        ];

        let blue = iced_native::row![
            widget::text("Fog blue"),
            widget::slider(COMPONENT_RANGE, state.fog.color.blue, Message::FogBlue).step(0.01),
        ];

        let alpha = iced_native::row![
            widget::text("Fog alpha"),
            widget::slider(COMPONENT_RANGE, state.fog.alpha, Message::FogAlpha).step(0.01),
        ];

        let fog_start = iced_native::row![
            widget::text("Fog start distance"),
            widget::slider(FOG_RANGE, state.fog_start, Message::FogStart).step(0.01),
        ];

        let fog_end = iced_native::row![
            widget::text("Fog end distance"),
            widget::slider(FOG_RANGE, state.fog_end, Message::FogEnd).step(0.01),
        ];

        let fog_clip = iced_native::row![
            widget::text("Fog clip distance"),
            widget::slider(FOG_RANGE, state.fog_clip, Message::FogClip).step(0.01),
        ];

        let bloom = iced_native::row![
            widget::text("Bloom intensity"),
            widget::slider(BLOOM_RANGE, state.bloom, Message::Bloom).step(0.01),
        ];

        let exposure_min = iced_native::row![
            widget::text("Exposure min"),
            widget::slider(EXPOSURE_RANGE, state.exposure_min, Message::ExposureMin).step(0.01),
        ];

        let exposure_max = iced_native::row![
            widget::text("Exposure max"),
            widget::slider(EXPOSURE_RANGE, state.exposure_max, Message::ExposureMax).step(0.01),
        ];

        let content = iced_native::column![
            anti_aim,
            fake_lag,
            thirdperson,
            red,
            green,
            blue,
            alpha,
            fog_start,
            fog_end,
            fog_clip,
            bloom,
            exposure_min,
            exposure_max,
        ]
        .spacing(15);

        let menu = widget::container(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y()
            .padding(20)
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
        background(Color::from_rgba8(0x00, 0x00, 0x00, 0.7))
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
