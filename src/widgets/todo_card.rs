use iced::border::Radius;
use iced::overlay::menu;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::AppMessage;
use crate::theme::ThemeColors;
use crate::todo::{Todo, TodoMessage, TodoPriority, TodoStatus, TodoTitleState};

pub fn todo_card<'a>(
    todo: &'a Todo,
    theme_colors: ThemeColors,
    index: usize,
    window_ratio: f32,
    todo_edit_buffer: &'a str,
) -> iced::Element<'a, TodoMessage> {
    let checkbox =
        iced::widget::checkbox(todo.status == TodoStatus::Completed).on_toggle(move |completed| {
            TodoMessage::Toggled(
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
        .on_press(TodoMessage::ShowEdit(index))
        .style(move |_, status| edit_button_style(theme_colors, status, window_ratio));

    let edit_input_text = iced::widget::text_input("", todo_edit_buffer)
        .on_input(TodoMessage::EditChanged)
        .style(move |_, status| edit_input_text_style(theme_colors, status, window_ratio));

    let priority_card = iced::widget::PickList::new(
        [TodoPriority::High, TodoPriority::Medium, TodoPriority::Low],
        Some(todo.priority),
        move |priority| TodoMessage::PriorityChanged(index, priority),
    )
    .style(move |_, status| picklist_style(theme_colors, todo.priority, status, window_ratio))
    .menu_style(move |_| picklist_menu_style(theme_colors))
    .width(Length::Shrink)
    .padding([4.0 * window_ratio, 10.0 * window_ratio]);

    let priority_card_wrapper = container(priority_card).width(Length::Shrink);

    let save_button = button(text("Save"))
        .on_press(TodoMessage::Edit(index))
        .style(move |_, status| edit_button_style(theme_colors, status, window_ratio));

    let cancel_button = button(text("Cancel"))
        .on_press(TodoMessage::CancelEdit(index))
        .style(move |_, status| edit_button_style(theme_colors, status, window_ratio));

    let delete_button = button(text("Delete"))
        .on_press(TodoMessage::Delete(index))
        .style(move |_, status| delete_button_style(theme_colors, status, window_ratio));

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
                priority_card_wrapper,
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
        .style(move |_| todo_style(theme_colors, todo))
        .into()
}

#[inline]
fn todo_style(theme_colors: ThemeColors, todo: &Todo) -> iced::widget::container::Style {
    let bg = if todo.status == TodoStatus::Completed {
        theme_colors.task_completed_bg
    } else {
        theme_colors.task_bg
    };

    let task_border_color = if todo.status == TodoStatus::Completed {
        theme_colors.task_completed_border
    } else {
        theme_colors.task_border
    };

    let text_color = if todo.status == TodoStatus::Completed {
        theme_colors.task_completed_text
    } else {
        theme_colors.task_text
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
    theme_colors: ThemeColors,
    status: Status,
    window_ratio: f32,
) -> iced::widget::button::Style {
    let edit_bg = theme_colors.amber_bg;
    let edit_bg_hover = theme_colors.amber_bg_hover;
    let edit_text = theme_colors.amber_text;
    let edit_border = theme_colors.amber_border;

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

#[inline]
pub fn picklist_style(
    colors: ThemeColors,
    priority: TodoPriority,
    status: iced::widget::pick_list::Status,
    window_ratio: f32,
) -> iced::widget::pick_list::Style {
    let (priority_bg, priority_text, priority_border) = priority.colors(colors);

    let border_color = match status {
        iced::widget::pick_list::Status::Active => priority_border,

        iced::widget::pick_list::Status::Hovered => priority_border,

        iced::widget::pick_list::Status::Opened { .. } => priority_border,
    };

    iced::widget::pick_list::Style {
        text_color: priority_text,
        placeholder_color: colors.picklist_placeholder,
        handle_color: priority_text,
        background: priority_bg.into(),
        border: Border {
            color: border_color,
            width: 1.0 * window_ratio,
            radius: Radius::new(999.0 * window_ratio),
        },
    }
}

#[inline]
pub fn picklist_menu_style(colors: ThemeColors) -> menu::Style {
    menu::Style {
        text_color: colors.text_main,
        background: colors.picklist_menu_bg.into(),
        border: Border {
            color: colors.picklist_menu_border,
            width: 1.0,
            radius: Radius::new(8.0),
        },
        selected_text_color: colors.picklist_menu_selected_text,
        selected_background: colors.picklist_menu_selected_bg.into(),
        shadow: iced::Shadow {
            color: Color::from_rgba8(0, 0, 0, 0.1),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    }
}

fn edit_input_text_style(
    theme_colors: ThemeColors,
    status: iced::widget::text_input::Status,
    window_ratio: f32,
) -> iced::widget::text_input::Style {
    let bg = theme_colors.input_bg;
    let bg_hover = theme_colors.input_bg_hover;

    let border = theme_colors.input_border;
    let border_hover = theme_colors.input_border_hover;
    let border_focused = theme_colors.input_border_focused;

    let text = theme_colors.text_main;
    let placeholder = theme_colors.text_placeholder;
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
    theme_colors: ThemeColors,
    status: Status,
    window_ratio: f32,
) -> iced::widget::button::Style {
    let bg = theme_colors.red_bg;
    let hover = theme_colors.red_bg_hover;
    let text = theme_colors.red_text;

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
