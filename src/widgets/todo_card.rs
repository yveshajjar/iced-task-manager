use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::tasks::{TodoItem, TodoStatus};

const TASK_BACKGROUND: Color = Color::from_rgb8(248, 250, 252);
const TASK_BORDER: Color = Color::from_rgb8(226, 232, 240);
const TASK_TEXT: Color = Color::from_rgb8(30, 41, 59);

const TASK_COMPLETED_BACKGROUND: Color = Color::from_rgb8(250, 250, 250);
const TASK_COMPLETED_BORDER: Color = Color::from_rgb8(229, 231, 235);
const TASK_COMPLETED_TEXT: Color = Color::from_rgb8(148, 163, 184);

pub fn todo_card<'a>(
    todo: &'a TodoItem,
    index: usize,
    window_ratio: f32,
) -> iced::Element<'a, AppMessage> {
    let checkbox =
        iced::widget::checkbox(todo.status == TodoStatus::Completed).on_toggle(move |completed| {
            AppMessage::TodoToggled(
                index,
                if completed {
                    TodoStatus::Completed
                } else {
                    TodoStatus::Active
                },
            )
        });

    container(
        row![
            checkbox,
            text(&todo.title),
            space().width(Length::Fill),
            button(text("Delete"))
                .on_press(AppMessage::DeleteTodo(index))
                .style(move |theme, status| todo_delete_button(theme, status, window_ratio)),
        ]
        .spacing(5.0 * window_ratio)
        .align_y(iced::alignment::Vertical::Center)
        .padding(6.0 * window_ratio),
    )
    .center_x(Length::Fixed(360.0 * window_ratio))
    .center_y(Length::Fixed(30.0 * window_ratio))
    .style(|theme| todo_style(theme, todo))
    .into()
}

#[inline]
fn todo_style(theme: &Theme, todo: &TodoItem) -> iced::widget::container::Style {
    let bg = if todo.status == TodoStatus::Completed {
        TASK_COMPLETED_BACKGROUND
    } else {
        TASK_BACKGROUND
    };

    let task_border_color = if todo.status == TodoStatus::Completed {
        TASK_COMPLETED_BORDER
    } else {
        TASK_BORDER
    };

    let text_color = if todo.status == TodoStatus::Completed {
        TASK_COMPLETED_TEXT
    } else {
        TASK_TEXT
    };

    iced::widget::container::Style {
        background: Some(bg.into()),
        border: Border {
            radius: Radius::from(5.0),
            color: task_border_color,
            ..Default::default()
        },
        text_color: Some(text_color),
        ..Default::default()
    }
}

#[inline]
fn todo_delete_button(
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
