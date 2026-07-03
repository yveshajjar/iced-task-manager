use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::tasks::{TodoItem, TodoStatus};

pub fn input_bar<'a>(todo_input: &str, window_ratio: f32) -> iced::Element<'a, AppMessage> {
    let input_text = iced::widget::text_input("Enter new todo", &todo_input)
        .on_input(AppMessage::TodoInputChanged)
        .style(input_text_style);

    row![
        input_text,
        iced::widget::button("Add").on_press(AppMessage::AddTodo)
    ]
    .spacing(5.0 * window_ratio)
    .width(Length::Fixed(360.0 * window_ratio))
    .height(Length::Fixed(30.0 * window_ratio))
    .into()
}

#[inline]
fn input_text_style(
    theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let bg = Color::from_rgb8(248, 250, 252);
    let border = Color::from_rgb8(226, 232, 240);
    let text = Color::from_rgb8(30, 41, 59);
    let placeholder = Color::from_rgb8(148, 163, 184);

    iced::widget::text_input::Style {
        background: bg.into(),
        border: Border {
            radius: Radius::from(5.0),
            color: border,
            ..Default::default()
        },
        placeholder,
        value: text,
        icon: Color::WHITE,
        selection: Color::WHITE,
    }
}
