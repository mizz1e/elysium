use iced_native::{overlay, renderer, text, widget};
use num_traits::FromPrimitive;
use std::borrow::Cow;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub fn slider<'a, T, Message, Renderer>(
    label: impl Display,
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> widget::Row<'a, Message, Renderer>
where
    T: Copy + Display + From<u8> + FromPrimitive + Into<f64> + PartialOrd + 'a,
    Message: Clone + 'a,
    Renderer: renderer::Renderer + text::Renderer + 'a,
    Renderer::Theme: widget::slider::StyleSheet + widget::text::StyleSheet,
{
    let label = format!("{label} ({value})");
    let text = widget::text(label);
    let slider = widget::slider(range, value, on_change);

    iced_native::row![text, slider]
}

pub fn pick_list<'a, Message, Renderer, T>(
    label: impl ToString,
    options: impl Into<Cow<'a, [T]>>,
    selected: Option<T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> widget::Row<'a, Message, Renderer>
where
    T: Clone + Eq + ToString + 'static,
    [T]: ToOwned<Owned = Vec<T>>,
    Message: 'a,
    Renderer: text::Renderer + 'a,
    Renderer::Theme: overlay::menu::StyleSheet + widget::container::StyleSheet + widget::pick_list::StyleSheet + widget::text::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced_native::overlay::menu::StyleSheet>::Style: From<<<Renderer as iced_native::Renderer>::Theme as iced_native::widget::pick_list::StyleSheet>::Style>,
    <Renderer as iced_native::Renderer>::Theme: iced_native::widget::scrollable::StyleSheet
{
    let text = widget::text(label);
    let pick_list = widget::pick_list(options, selected, on_selected);

    iced_native::row![text, pick_list]
}
