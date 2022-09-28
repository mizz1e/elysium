use elysium_sdk::Flow;
use iced_glow::Renderer;
use iced_native::alignment::{Horizontal, Vertical};
use iced_native::widget::{container, text};
use iced_native::{widget, Alignment, Command, Element, Length, Program};
use std::os::unix::ffi::OsStrExt;

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
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        view(self).into()
    }
}

fn view<'a, Message, Renderer>(hud: &Hud) -> widget::Container<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::Renderer + iced_native::text::Renderer + 'a,
    Renderer::Theme: container::StyleSheet + text::StyleSheet,
{
    let state = crate::State::get();

    let mut status = String::new();
    let mut status1 = String::new();
    let mut status2 = String::new();

    use std::time::Duration;
    use ubyte::ByteUnit;

    fn display_network_status(
        direction: &'static str,
        latency: Duration,
        packet_rate: u32,
        data_rate: ByteUnit,
    ) -> String {
        let (whole, frac, suffix, _unit) = data_rate.repr();
        let rate = whole as f64 + frac;

        format!(" {direction} {latency:.2?} {packet_rate} pkt/s {rate:.2} {suffix}/s")
    }

    if let Some(interfaces) = state.interfaces.as_ref() {
        if let Some(level_name) = interfaces.engine.level_name() {
            let level_name = String::from_utf8_lossy(level_name.as_bytes());

            status = format!("{level_name}");
        }

        if let Some(network_channel) = interfaces.engine.network_channel() {
            if let Some(address) = network_channel.address() {
                status = format!("{status} on {address}");
            } else {
                status = format!("{status} on local server");
            }

            let (latency_incoming, latency_outgoing) = network_channel.average_latency_pair();

            let packets_incoming = network_channel.avg_packets(Flow::Incoming).trunc() as u32;
            let packets_outgoing = network_channel.avg_packets(Flow::Outgoing).trunc() as u32;

            let data_incoming =
                ubyte::ByteUnit::Byte(network_channel.avg_data(Flow::Incoming).trunc() as u64);

            let data_outgoing =
                ubyte::ByteUnit::Byte(network_channel.avg_data(Flow::Outgoing).trunc() as u64);

            status1 =
                display_network_status(">", latency_incoming, packets_incoming, data_incoming);
            status2 =
                display_network_status("<", latency_outgoing, packets_outgoing, data_outgoing);
        }
    }

    let top_left = widget::container(iced_native::column![
        widget::text(status),
        widget::text(" "),
        widget::text(status1),
        widget::text(status2)
    ])
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
