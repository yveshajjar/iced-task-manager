use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::{App, AppMessage};
use crate::theme::{AppTheme, ThemeColors};
use crate::todo::TodoMessage;

pub fn search_bar<'a>(app: &'a App, todo_search: &str) -> iced::Element<'a, TodoMessage> {
    let window_ratio = app.window_ratio;
    let theme_colors = app.theme.colors();

    let search_input = iced::widget::text_input("Search tasks...", todo_search)
        .on_input(TodoMessage::SearchChanged)
        .style(move |_, status| input_text_style(theme_colors, status));

    container(search_input)
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
