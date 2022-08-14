use iced_glow::Renderer;
use iced_native::widget::{
    button, scrollable, slider, text_input, Button, Checkbox, Column, Container, ProgressBar, Row,
    Rule, Scrollable, Slider, Space, Text, TextInput, Toggler,
};
use iced_native::{Alignment, Command, Element, Length, Program};

#[derive(Default)]
pub struct Controls {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
    menu_visibility: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    MenuVisibility(bool),
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
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::MenuVisibility(value) => self.menu_visibility = value,
        }

        Command::none()
    }

    #[inline]
    fn view(&self) -> Element<Message, Renderer> {
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Rule::horizontal(38))
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(Rule::vertical(38))
                    .push(Column::new().width(Length::Shrink).spacing(20)),
            );

        let menu = Container::new(content)
            .width(Length::Units(800))
            .height(Length::Units(640))
            .center_x()
            .center_y();

        Container::new(menu)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
