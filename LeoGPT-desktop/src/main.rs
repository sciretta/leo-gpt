use iced::Length::Fill;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Background, Border, Color, Element, Theme};

struct State {
    value: i8,
    route: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: 10,
            route: "counter".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    GoToSettings,
}

fn view(state: &State) -> Element<'_, Message> {
    let col1 = column![button("Increment").on_press(Message::Increment),]
        .spacing(10)
        .width(Fill)
        .align_x(Horizontal::Center);

    let col2 = column![text(format!("Counter: {}", state.value)),].spacing(10);

    let col3 = column![
        button("Decrement").on_press(Message::Decrement),
        button("Settings").on_press(Message::GoToSettings),
    ]
    .spacing(10)
    .width(Fill)
    .align_x(Horizontal::Center);

    let layout = row![col1, col2, col3]
        .spacing(20)
        .width(Fill)
        .align_y(Vertical::Center);

    container(layout)
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .padding(20)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::BLACK)),
            border: Border {
                width: 1.0,
                color: Color::WHITE,
                radius: 0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn view_settings(state: &State) -> Element<'_, Message> {
    container(text("Settings Page"))
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .padding(20)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::BLACK)),
            border: Border {
                width: 1.0,
                color: Color::WHITE,
                radius: 0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn router(state: &State) -> Element<'_, Message> {
    match state.route.as_str() {
        "counter" => view(state),
        "settings" => view_settings(state),
        _ => view(state),
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.value += 10,
        Message::Decrement => state.value -= 10,
        Message::GoToSettings => state.route = "settings".to_string(),
    }
}

pub fn main() -> iced::Result {
    iced::run(update, router)
}
