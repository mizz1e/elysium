use core::fmt;
use core::time::Duration;
use elysium_sdk::Flow;
use iced_glow::Renderer;
use iced_native::alignment::{Horizontal, Vertical};
use iced_native::widget::{container, text};
use iced_native::{widget, Alignment, Command, Element, Length, Program};
use std::net::SocketAddr;
use ubyte::ByteUnit;

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
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        view(self).into()
    }
}

fn collect_column<'a, Message, Renderer, I, E>(iter: I) -> widget::Column<'a, Message, Renderer>
where
    I: IntoIterator<Item = E>,
    E: Into<Element<'a, Message, Renderer>>,
{
    let mut column = widget::Column::new();

    for item in iter {
        column = column.push(item);
    }

    column
}

struct Info {
    level_name: Option<String>,
    location: Option<String>,
    address: Option<SocketAddr>,
    net_stats: Option<NetStats>,
}

impl Info {
    fn new(state: &crate::State) -> Self {
        let mut level_name = None;
        let mut location = None;
        let mut address = None;
        let mut net_stats = None;

        if let Some(interfaces) = state.interfaces.as_ref() {
            if let Some(name) = interfaces.engine.level_name() {
                level_name = Some(
                    name.to_string_lossy()
                        .replace(char::REPLACEMENT_CHARACTER, "?"),
                );
            }

            if let Some(name) = &state.location {
                location = Some(
                    name.to_string_lossy()
                        .replace(char::REPLACEMENT_CHARACTER, "?"),
                );
            }

            if let Some(network_channel) = interfaces.engine.network_channel() {
                address = network_channel.address();

                let (latency_incoming, latency_outgoing) = network_channel.average_latency_pair();

                let packets_incoming = network_channel.avg_packets(Flow::Incoming).trunc() as u32;
                let packets_outgoing = network_channel.avg_packets(Flow::Outgoing).trunc() as u32;

                let data_incoming =
                    ubyte::ByteUnit::Byte(network_channel.avg_data(Flow::Incoming).trunc() as u64);

                let data_outgoing =
                    ubyte::ByteUnit::Byte(network_channel.avg_data(Flow::Outgoing).trunc() as u64);

                net_stats = Some(NetStats {
                    incoming: FlowStats {
                        direction: '>',
                        latency: latency_incoming,
                        packet_rate: packets_incoming,
                        data_rate: data_incoming,
                    },
                    outgoing: FlowStats {
                        direction: '<',
                        latency: latency_outgoing,
                        packet_rate: packets_outgoing,
                        data_rate: data_outgoing,
                    },
                });
            }
        }

        Self {
            level_name,
            location,
            address,
            net_stats,
        }
    }
}

impl fmt::Display for Info {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            level_name,
            location,
            address,
            net_stats,
        } = self;

        if let Some(level_name) = level_name {
            write!(fmt, "{level_name}")?;

            if let Some(location) = location {
                if !location.is_empty() {
                    write!(fmt, " at {location}")?;
                }
            }

            if let Some(address) = address {
                write!(fmt, " on {address}")?;
            }

            write!(fmt, "\n\n")?;
        }

        if let Some(net_stats) = net_stats {
            write!(fmt, "{}\n", net_stats.incoming)?;
            write!(fmt, "{}\n", net_stats.outgoing)?;
        }

        Ok(())
    }
}

struct FlowStats {
    direction: char,
    latency: Duration,
    packet_rate: u32,
    data_rate: ByteUnit,
}

struct NetStats {
    incoming: FlowStats,
    outgoing: FlowStats,
}

impl fmt::Display for FlowStats {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            direction,
            latency,
            packet_rate,
            data_rate,
        } = self;
        let (whole, frac, suffix, _unit) = data_rate.repr();
        let rate = whole as f64 + frac;

        write!(
            fmt,
            " {direction} {latency:.2?} {packet_rate} pkt/s {rate:.2} {suffix}/s"
        )
    }
}

fn view<'a, Message, Renderer>(_hud: &Hud) -> widget::Container<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::Renderer + iced_native::text::Renderer + 'a,
    Renderer::Theme: container::StyleSheet + text::StyleSheet,
{
    let state = crate::State::get();
    let info = Info::new(&state);

    let status = info.to_string();

    let iter = status
        .lines()
        .map(|line| if line.is_empty() { " " } else { line })
        .map(widget::text);

    let top_left = collect_column(iter);
    let top_left = widget::container(top_left)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .height(Length::Fill)
        .width(Length::Fill);

    let top_centre = widget::text(" ");
    let top_right = widget::text(" ");

    let centre_left = widget::text(" ");
    let centre_centre = widget::text("+");
    let centre_right = widget::text(" ");

    let bottom_left = widget::text(" ");
    let bottom_centre = widget::text(" ");
    let bottom_right = widget::text(" ");

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
