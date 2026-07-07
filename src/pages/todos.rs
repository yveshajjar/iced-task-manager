use iced::border::Radius;
use iced::overlay::menu;
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use strum::IntoEnumIterator;

use crate::app::AppMessage::TodoSortChanged;
use crate::app::{App, AppMessage};
use crate::theme::ThemeColors;
use crate::widgets::input_bar::input_bar;
use crate::widgets::todo_card::todo_card;

use crate::storage;
use crate::todo::{Todo, TodoSort};
use crate::todo::{TodoFilter, TodoPriority};

pub fn todos_page<'a>(app: &'a App, current_filter: TodoFilter) -> iced::Element<'a, AppMessage> {
    let theme_colors = app.theme.colors();

    let mut todos_raw: Vec<_> = app
        .todos
        .iter()
        .enumerate()
        .filter(|(_, todo)| current_filter.matches(&todo.status))
        .collect();

    let has_todos = !todos_raw.is_empty();

    match app.todo_sort {
        TodoSort::Created => {}
        TodoSort::PriorityHighFirst => {
            todos_raw.sort_by_key(|(_, todo)| std::cmp::Reverse(todo.priority));
        }
        TodoSort::PriorityLowFirst => {
            todos_raw.sort_by_key(|(_, todo)| todo.priority);
        }
        TodoSort::CompletedLast => {
            todos_raw.sort_by_key(|(_, todo)| todo.status);
        }
    }

    let todos_card_vector: Vec<_> = todos_raw
        .into_iter()
        .map(|(index, todo)| {
            todo_card(
                todo,
                theme_colors,
                index,
                app.window_ratio,
                &app.todo_edit_buffer,
            )
        })
        .collect();

    let todos_column = column(todos_card_vector)
        .spacing(2.0 * app.window_ratio)
        .width(Length::Fixed(320.0 * app.window_ratio))
        .height(Length::Fill);

    let todos_scrollable = iced::widget::scrollable(todos_column)
        .width(Length::Fixed(370.0 * app.window_ratio))
        .height(Length::Fixed(460.0 * app.window_ratio))
        .style(scrollable_style);

    let input_bar = input_bar(app, &app.todo_input_buffer);

    let empty_text = match current_filter {
        TodoFilter::All => "No todos yet. Add your first task above.",
        TodoFilter::Active => "No active todos.",
        TodoFilter::Completed => "No completed todos yet.",
    };

    let empty_state_text = text(empty_text)
        .size(15.0 * app.window_ratio)
        .color(theme_colors.text_main);

    let todos_content = if has_todos {
        container(todos_scrollable)
    } else {
        container(empty_state_text)
    };

    let sort_picklist = iced::widget::pick_list(
        TodoSort::iter().collect::<Vec<_>>(),
        Some(app.todo_sort),
        TodoSortChanged,
    )
    .width(Length::Shrink)
    .style(move |_, _| picklist_style(theme_colors, app.window_ratio))
    .menu_style(move |_| picklist_menu_style(theme_colors));

    let sort_picklist_wrapper = container(sort_picklist).width(Length::Shrink);

    column![
        text("My Tasks")
            .size(28.0 * app.window_ratio)
            .color(theme_colors.text_main),
        container(
            row![
                text("Sort by: ")
                    .size(16.0 * app.window_ratio)
                    .color(theme_colors.text_main),
                sort_picklist_wrapper
            ]
            .align_y(iced::alignment::Vertical::Center)
        )
        .align_y(iced::alignment::Vertical::Center),
        input_bar,
        todos_content,
    ]
    .width(Length::Fill)
    .align_x(iced::alignment::Horizontal::Center)
    .spacing(18.0 * app.window_ratio)
    .into()
}

#[inline]
fn scrollable_style(
    _theme: &Theme,
    _status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    iced::widget::scrollable::Style {
        auto_scroll: AutoScroll {
            background: Color::from_rgb8(226, 232, 240).into(),
            border: Border {
                radius: Radius::from(5.0),
                color: Color::from_rgb8(226, 232, 240),
                ..Default::default()
            },
            icon: Color::from_rgb8(148, 163, 184),
            shadow: Shadow {
                ..Default::default()
            },
        },
        container: iced::widget::container::Style {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                radius: Radius::from(5.0),
                color: Color::from_rgb8(226, 232, 240),
                ..Default::default()
            },
            ..Default::default()
        },
        gap: None,
        horizontal_rail: Rail {
            background: Some(Color::from_rgb8(226, 232, 240).into()),
            border: Border {
                radius: Radius::from(5.0),
                color: Color::from_rgb8(226, 232, 240),
                ..Default::default()
            },
            scroller: Scroller {
                background: Color::from_rgb8(148, 163, 184).into(),
                border: Border {
                    radius: Radius::from(5.0),
                    color: Color::from_rgb8(148, 163, 184),
                    ..Default::default()
                },
            },
        },
        vertical_rail: Rail {
            background: Some(Color::from_rgb8(226, 232, 240).into()),
            border: Border {
                radius: Radius::from(5.0),
                color: Color::from_rgb8(226, 232, 240),
                ..Default::default()
            },
            scroller: Scroller {
                background: Color::from_rgb8(148, 163, 184).into(),
                border: Border {
                    radius: Radius::from(5.0),
                    color: Color::from_rgb8(148, 163, 184),
                    ..Default::default()
                },
            },
        },
    }
}

#[inline]
pub fn picklist_style(colors: ThemeColors, window_ratio: f32) -> iced::widget::pick_list::Style {
    iced::widget::pick_list::Style {
        text_color: colors.text_main,
        placeholder_color: colors.picklist_placeholder,
        handle_color: colors.text_main,
        background: colors.picklist_menu_bg.into(),
        border: Border {
            color: colors.picklist_menu_border,
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
