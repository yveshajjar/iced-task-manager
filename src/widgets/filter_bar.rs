use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::tasks::{TodoFilter, TodoItem, TodoStatus};

pub fn filter_bar<'a>(
    window_ratio: f32,
    current_filter: &'a TodoFilter,
) -> iced::Element<'a, AppMessage> {
    let filter_row = row![
        button(text("All"))
            .on_press(AppMessage::TodoFilterChanged(TodoFilter::All))
            .style(move |_, status| button_style(
                TodoFilter::All,
                current_filter,
                status,
                window_ratio
            )),
        button(text("Active"))
            .on_press(AppMessage::TodoFilterChanged(TodoFilter::Active))
            .style(move |_, status| button_style(
                TodoFilter::Active,
                current_filter,
                status,
                window_ratio
            )),
        button(text("Completed"))
            .on_press(AppMessage::TodoFilterChanged(TodoFilter::Completed))
            .style(move |_, status| button_style(
                TodoFilter::Completed,
                current_filter,
                status,
                window_ratio
            )),
    ]
    .spacing(5.0 * window_ratio)
    .height(Length::Fixed(30.0 * window_ratio));

    container(filter_row)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}

#[inline]
fn button_style(
    button_filter: TodoFilter,
    current_filter: &TodoFilter,
    status: Status,
    window_ratio: f32,
) -> Style {
    // Selected filter
    let selected_bg = Color::from_rgb8(239, 246, 255);
    let selected_border = Color::from_rgb8(147, 197, 253);
    let selected_text = Color::from_rgb8(37, 99, 235);

    // Inactive filter
    let inactive_bg = Color::from_rgb8(255, 255, 255);
    let inactive_border = Color::from_rgb8(226, 232, 240);
    let inactive_text = Color::from_rgb8(100, 116, 139);

    // Hover inactive
    let inactive_bg_hover = Color::from_rgb8(241, 245, 249);
    let inactive_border_hover = Color::from_rgb8(203, 213, 225);

    let bg = if button_filter == *current_filter {
        selected_bg
    } else {
        inactive_bg
    };

    let border_color = if button_filter == *current_filter {
        selected_border
    } else {
        inactive_border
    };

    let text = if button_filter == *current_filter {
        selected_text
    } else {
        inactive_text
    };

    Style {
        background: if status == Status::Hovered {
            Some(inactive_bg_hover.into())
        } else {
            Some(bg.into())
        },
        border: Border {
            color: if status == Status::Hovered {
                inactive_border_hover
            } else {
                border_color
            },
            radius: Radius::new(5.0 * window_ratio),
            ..Default::default()
        },
        text_color: text,
        ..Default::default()
    }
}
