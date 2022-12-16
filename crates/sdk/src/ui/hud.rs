use super::{Element, Layer, Message};
use crate::{
    config, config::Pitch, engine, global, Color, Config, IClientEntity, WalkingAnimation,
};
use iced_glow::Renderer as GlowRenderer;
use iced_native::{
    alignment::{Alignment, Horizontal, Vertical},
    column, row, widget, Command, Length, Program,
};
use std::borrow::Cow;

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
        let mut top_left = String::new();
        let mut bottom_left = String::new();

        if let Some(client_state) = crate::ClientState::get() {
            top_left += &format!("{:?}\n\n", client_state.sign_on_state());

            if let Some(net_channel) = client_state.net_channel() {
                let info = net_channel.info();

                top_left += &format!("{}\n", info.display());
            }

            top_left += &format!("FL = {}", client_state.choked_commands());
        }

        if let Some(local_player) = IClientEntity::local_player() {
            if let Some(location_name) = local_player.location_name() {
                top_left.insert_str(0, &format!("{}\n", location_name.to_string_lossy()));
            }

            let armor = local_player.armor_value();
            let health = local_player.health();

            bottom_left = format!("health {health} armor {armor}");
        };

        let first = row(
            Vertical::Top,
            widget::text(top_left),
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
            widget::text(bottom_left),
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
