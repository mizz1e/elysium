use crate::config::Pitch;
use crate::{config, engine, global, Color, Config, IClientEntity, WalkingAnimation};
use iced_native::alignment::{Alignment, Horizontal, Vertical};
use iced_native::{column, row, widget, Command, Element, Length, Program};

pub struct Hud;

#[derive(Clone, Debug)]
pub enum Message {
    None,
}

impl Program for Hud {
    type Renderer = iced_glow::Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Message, iced_glow::Renderer> {
        view()
    }
}

fn cell<'a, E>(element: E) -> widget::Container<'a, Message, iced_glow::Renderer>
where
    E: Into<Element<'a, Message, iced_glow::Renderer>>,
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
) -> widget::Row<'a, Message, iced_glow::Renderer>
where
    L: Into<Element<'a, Message, iced_glow::Renderer>>,
    C: Into<Element<'a, Message, iced_glow::Renderer>>,
    R: Into<Element<'a, Message, iced_glow::Renderer>>,
{
    let left = cell(left).align_x(Horizontal::Left).align_y(vertical);
    let center = cell(center).align_x(Horizontal::Center).align_y(vertical);
    let right = cell(right).align_x(Horizontal::Right).align_y(vertical);

    row![left, center, right]
        .height(Length::Fill)
        .width(Length::Fill)
}

fn view<'a>() -> Element<'a, Message, iced_glow::Renderer> {
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
