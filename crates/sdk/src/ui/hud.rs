use super::{Element, Layer, Message};
use crate::{
    config, config::Pitch, engine, global, Color, Config, IClientEntity, WalkingAnimation,
};
use iced_glow::Renderer as GlowRenderer;
use iced_native::{
    alignment::{Alignment, Horizontal, Vertical},
    column, row, widget, Command, Length, Program,
};

pub struct Hud;

impl Layer for Hud {
    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_> {
        view()
    }
}

fn cell<'a, E>(element: E) -> widget::Container<'a, Message, GlowRenderer>
where
    E: Into<Element<'a>>,
{
    widget::container(element)
        .height(Length::Fill)
        .width(Length::Fill)
}

fn row<'a, L, C, R>(
    vertical: Vertical,
    left: L,
    center: C,
    right: R,
) -> widget::Row<'a, Message, GlowRenderer>
where
    L: Into<Element<'a>>,
    C: Into<Element<'a>>,
    R: Into<Element<'a>>,
{
    let left = cell(left).align_x(Horizontal::Left).align_y(vertical);
    let center = cell(center).align_x(Horizontal::Center).align_y(vertical);
    let right = cell(right).align_x(Horizontal::Right).align_y(vertical);

    row![left, center, right]
        .height(Length::Fill)
        .width(Length::Fill)
}

fn view<'a>() -> Element<'a> {
    global::with_app(|app| {
        let (living_status, location_name) =
            if let Some(local_player) = IClientEntity::local_player() {
                let armor = local_player.armor_value();
                let health = local_player.health();
                let location_name = local_player.location_name();

                let living_status = widget::text(format!("health {health} armor {armor}"));
                let location_name = if let Some(location_name) = location_name {
                    widget::text(location_name.to_string_lossy())
                } else {
                    widget::text(" ")
                };

                (living_status, location_name)
            } else {
                (widget::text(" "), widget::text(" "))
            };

        let first = row(
            Vertical::Top,
            location_name,
            column![widget::text("2:45 left"), widget::text("8 to 0")]
                .align_items(Alignment::Center),
            widget::text("bot sample killed bot sample"),
        );

        let second = row(
            Vertical::Center,
            widget::text(" "),
            widget::text("+"),
            widget::text(" "),
        );

        let third = row(
            Vertical::Bottom,
            living_status,
            widget::text(" "),
            widget::text("10/60 bullets"),
        );

        column![first, second, third]
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(40)
            .into()
    })
}
