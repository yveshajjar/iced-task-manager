use iced::border::Radius;
use iced::overlay::menu;
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use strum::IntoEnumIterator;

use crate::app::{App, AppMessage};
use crate::theme::ThemeColors;
use crate::widgets::input_bar::input_bar;
use crate::widgets::search_bar::search_bar;
use crate::widgets::todo_card::todo_card;

use crate::storage;
use crate::todo::TodoMessage;
use crate::todo::{Todo, TodoSort};
use crate::todo::{TodoFilter, TodoPriority};

pub fn todos_page<'a>(app: &'a App, current_filter: TodoFilter) -> iced::Element<'a, AppMessage> {
    let theme_colors = app.theme.colors();

    let visible_todo_cards = visible_todo_cards(app, current_filter, &app.todo_search_buffer);

    let has_todos = !visible_todo_cards.is_empty();

    let todos_column = column(visible_todo_cards)
        .spacing(2.0 * app.window_ratio)
        .width(Length::Fixed(320.0 * app.window_ratio))
        .height(Length::Fill);

    let todos_scrollable = iced::widget::scrollable(todos_column)
        .width(Length::Fixed(370.0 * app.window_ratio))
        .height(Length::Fixed(460.0 * app.window_ratio))
        .style(scrollable_style);

    let input_bar = input_bar(app, &app.todo_input_buffer).map(AppMessage::Todo);

    let search_bar = search_bar(app, &app.todo_search_buffer).map(AppMessage::Todo);

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
        |sort| AppMessage::Todo(TodoMessage::SortChanged(sort)),
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
        container(input_bar).width(Length::Fixed(370.0 * app.window_ratio)),
        container(search_bar).width(Length::Fixed(370.0 * app.window_ratio)),
        todos_content,
    ]
    .width(Length::Fill)
    .align_x(iced::alignment::Horizontal::Center)
    .spacing(18.0 * app.window_ratio)
    .into()
}

fn visible_todo_cards<'a>(
    app: &'a App,
    current_filter: TodoFilter,
    search: &str,
) -> Vec<iced::Element<'a, AppMessage>> {
    let theme_colors = app.theme.colors();
    let search = search.trim().to_lowercase();

    let unfiltered_todos = app.todos.iter().enumerate();

    let filtered_todos = unfiltered_todos.filter(|(_, todo)| current_filter.matches(&todo.status));

    let mut searched_todos: Vec<_> = filtered_todos
        .filter(|(_, todo)| matches_search(todo, &search))
        .collect();

    sort_todos(&mut searched_todos, app.todo_sort);

    searched_todos
        .into_iter()
        .map(|(index, todo)| {
            todo_card(
                todo,
                theme_colors,
                index,
                app.window_ratio,
                &app.todo_edit_buffer,
            )
            .map(AppMessage::Todo)
        })
        .collect::<Vec<_>>()
}

fn sort_todos(todos: &mut Vec<(usize, &Todo)>, sort: TodoSort) {
    match sort {
        TodoSort::Created => {}
        TodoSort::PriorityHighFirst => {
            todos.sort_by_key(|(_, todo)| std::cmp::Reverse(todo.priority));
        }
        TodoSort::PriorityLowFirst => {
            todos.sort_by_key(|(_, todo)| todo.priority);
        }
        TodoSort::CompletedLast => {
            todos.sort_by_key(|(_, todo)| todo.status);
        }
    }
}

fn matches_search(todo: &Todo, normalized_search: &str) -> bool {
    normalized_search.is_empty() || todo.title.to_lowercase().contains(normalized_search)
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
