use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{Column, Space, button, column, container, row, space, text};
use iced::{Border, Color, Element, Length, Theme};
use strum::IntoEnumIterator;

use crate::app::AppMessage;
use crate::tasks::{TodoFilter, TodoItem, TodoStatus, TodoTitleState};

pub fn sidebar<'a>(
    window_ratio: f32,
    current_filter: TodoFilter,
    todos_count: Vec<usize>,
) -> Element<'a, AppMessage> {
    let filter_buttons: Vec<_> = TodoFilter::iter()
        .zip(&todos_count)
        .map(move |(filter, count)| {
            sidebar_filter_button(
                filter.to_string(),
                filter,
                *count,
                current_filter,
                window_ratio,
            )
        })
        .collect();

    let clear_button = button(container(text("Clear completed")).center(Length::Fill))
        .on_press(AppMessage::ClearCompletedTodos)
        .width(Length::Fill)
        .height(36.0 * window_ratio)
        .padding(0)
        .style(move |theme, status| clear_button_style(theme, status, window_ratio));

    let settings_button = button(text("Settings"))
        .on_press(AppMessage::SettingsPressed)
        .style(move |_, _| settings_button_style(window_ratio));

    let mut content = Column::new()
        .push(
            text("Iced Tasks")
                .size(22.0 * window_ratio)
                .color(Color::from_rgb8(30, 41, 59)),
        )
        .push(Space::new().height(24.0 * window_ratio))
        .extend(filter_buttons)
        .push(Space::new().height(Length::Fill));

    if todos_count[2] > 0 {
        content = content.push(clear_button);
    }

    content = content
        .push(settings_button)
        .spacing(8.0 * window_ratio)
        .padding([24.0 * window_ratio, 16.0 * window_ratio]);

    container(content)
        .width(Length::Fixed(230.0 * window_ratio))
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Color::from_rgb8(248, 250, 252).into()),
            border: Border {
                width: 1.0 * window_ratio,
                color: Color::from_rgb8(226, 232, 240),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

#[inline]
fn sidebar_filter_button<'a>(
    button_text: String,
    filter: TodoFilter,
    count: usize,
    current_filter: TodoFilter,
    window_ratio: f32,
) -> Element<'a, AppMessage> {
    let button_text = container(row![
        text(button_text),
        Space::new().width(Length::Fill),
        text(count)
    ])
    .padding([6.0 * window_ratio, 6.0 * window_ratio])
    .center(Length::Fill);

    button(button_text)
        .width(Length::Fill)
        .height(36.0 * window_ratio)
        .padding(0)
        .style(move |_, status| button_style(filter, current_filter, status, window_ratio))
        .on_press(AppMessage::TodoFilterChanged(filter))
        .into()
}

#[inline]
fn clear_button_style(
    theme: &iced::Theme,
    status: Status,
    window_ratio: f32,
) -> iced::widget::button::Style {
    let bg = Color::from_rgb8(254, 226, 226);
    let hover = Color::from_rgb8(254, 202, 202);
    let text = Color::from_rgb8(185, 28, 28);

    Style {
        background: Some(if status == iced::widget::button::Status::Hovered {
            hover.into()
        } else {
            bg.into()
        }),
        text_color: text,
        border: Border {
            radius: Radius::new(8.0 * window_ratio),
            color: Color::from_rgb8(254, 226, 226),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[inline]
fn button_style(
    button_filter: TodoFilter,
    current_filter: TodoFilter,
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

    let bg = if button_filter == current_filter {
        selected_bg
    } else {
        inactive_bg
    };

    let border_color = if button_filter == current_filter {
        selected_border
    } else {
        inactive_border
    };

    let text = if button_filter == current_filter {
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

#[inline]
fn settings_button_style(window_ratio: f32) -> Style {
    let bg = Color::from_rgb8(255, 255, 255);
    let border_color = Color::from_rgb8(226, 232, 240);
    let text_color = Color::from_rgb8(100, 116, 139);

    Style {
        background: Some(bg.into()),
        border: Border {
            color: border_color,
            radius: Radius::new(5.0 * window_ratio),
            ..Default::default()
        },
        text_color,
        ..Default::default()
    }
}
