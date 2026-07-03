use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::tasks::{TodoItem, TodoStatus, TodoTitleState};

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
    todo_edit_buffer: &'a str,
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

    let todo_title = text(&todo.title);

    let edit_button = button(text("Edit"))
        .on_press(AppMessage::ShowTodoEdit(index))
        .style(move |theme, status| edit_button_style(theme, status, window_ratio));

    let edit_input_text = iced::widget::text_input("", todo_edit_buffer)
        .on_input(AppMessage::TodoEditChanged)
        .style(move |theme, status| edit_input_text_style(theme, status, window_ratio));

    let save_button = button(text("Save"))
        .on_press(AppMessage::EditTodo(index))
        .style(move |theme, status| edit_button_style(theme, status, window_ratio));

    let cancel_button = button(text("Cancel"))
        .on_press(AppMessage::CancelEditTodo(index))
        .style(move |theme, status| edit_button_style(theme, status, window_ratio));

    let delete_button = button(text("Delete"))
        .on_press(AppMessage::DeleteTodo(index))
        .style(move |theme, status| delete_button_style(theme, status, window_ratio));

    let content = match todo.title_state {
        TodoTitleState::Editing => {
            row![
                checkbox,
                edit_input_text,
                space().width(Length::Fill),
                save_button,
                cancel_button,
            ]
        }
        TodoTitleState::Viewing => {
            row![
                checkbox,
                todo_title,
                space().width(Length::Fill),
                edit_button,
                delete_button
            ]
        }
    }
    .spacing(5.0 * window_ratio)
    .align_y(iced::alignment::Vertical::Center)
    .padding(6.0 * window_ratio);

    container(content)
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
fn edit_button_style(
    theme: &iced::Theme,
    status: Status,
    window_ratio: f32,
) -> iced::widget::button::Style {
    let edit_bg = Color::from_rgb8(255, 251, 235);
    let edit_bg_hover = Color::from_rgb8(254, 243, 199);
    let edit_text = Color::from_rgb8(146, 64, 14);
    let edit_border = Color::from_rgb8(245, 158, 11);

    Style {
        background: Some(if status == iced::widget::button::Status::Hovered {
            edit_bg_hover.into()
        } else {
            edit_bg.into()
        }),
        text_color: edit_text,
        border: Border {
            radius: Radius::new(8.0 * window_ratio),
            color: edit_border,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn edit_input_text_style(
    theme: &Theme,
    status: iced::widget::text_input::Status,
    window_ratio: f32,
) -> iced::widget::text_input::Style {
    let bg = Color::from_rgb8(248, 250, 252);
    let bg_hover = Color::from_rgb8(241, 245, 249);

    let border = Color::from_rgb8(226, 232, 240);
    let border_hover = Color::from_rgb8(203, 213, 225);
    let border_focused = Color::from_rgb8(147, 197, 253);

    let text = Color::from_rgb8(30, 41, 59);
    let placeholder = Color::from_rgb8(148, 163, 184);
    let icon = Color::from_rgb8(100, 116, 139);
    let selection = Color::from_rgb8(191, 219, 254);

    let (background, border_color) = match status {
        iced::widget::text_input::Status::Focused { .. } => (bg, border_focused),
        iced::widget::text_input::Status::Hovered => (bg_hover, border_hover),
        _ => (bg, border),
    };

    iced::widget::text_input::Style {
        background: background.into(),
        border: Border {
            radius: Radius::new(5.0 * window_ratio),
            width: 1.0 * window_ratio,
            color: border_color,
        },
        icon,
        placeholder,
        selection,
        value: text,
    }
}

#[inline]
fn delete_button_style(
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
