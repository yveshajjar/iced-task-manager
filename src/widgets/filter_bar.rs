use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::tasks::{TodoFilter, TodoItem, TodoStatus};

pub fn filter_bar<'a>(window_ratio: f32) -> iced::Element<'a, AppMessage> {
    let filter_row = row![
        button(text("All")).on_press(AppMessage::TodoFilterChanged(TodoFilter::All)),
        button(text("Active")).on_press(AppMessage::TodoFilterChanged(TodoFilter::Active)),
        button(text("Completed")).on_press(AppMessage::TodoFilterChanged(TodoFilter::Completed)),
    ]
    .spacing(5.0 * window_ratio)
    .height(Length::Fixed(30.0 * window_ratio));

    container(filter_row)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}
