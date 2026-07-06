use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::{App, AppMessage};
use crate::tasks::{TodoItem, TodoStatus};
use crate::theme::{AppTheme, ThemeColors};

pub fn input_bar<'a>(app: &'a App, todo_input: &str) -> iced::Element<'a, AppMessage> {
    let window_ratio = app.window_ratio;
    let theme = app.theme;
    let theme_colors = app.theme.colors();

    let input_text = iced::widget::text_input("Enter new todo", &todo_input)
        .on_input(AppMessage::TodoInputChanged)
        .style(move |_, status| input_text_style(theme_colors, status));

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
    theme_colors: ThemeColors,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let bg = theme_colors.input_bg;
    let border = theme_colors.input_border;
    let text = theme_colors.text_main;
    let placeholder = theme_colors.text_placeholder;

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
