use iced::border::Radius;
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Shadow, Theme, theme};

use crate::app::{App, AppMessage};
use crate::widgets::input_bar::input_bar;
use crate::widgets::todo_card::todo_card;

use crate::storage;
use crate::todo::TodoFilter;
use crate::todo::Todo;

pub fn todos_page<'a>(app: &'a App, current_filter: TodoFilter) -> iced::Element<'a, AppMessage> {
    let theme_colors = app.theme.colors();

    let todos_list: Vec<_> = app
        .todos
        .iter()
        .enumerate()
        .filter(|(_, todo)| current_filter.matches(&todo.status))
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

    let has_todos = !todos_list.is_empty();

    let todos_column = column(todos_list)
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

    column![
        text("My Tasks")
            .size(28.0 * app.window_ratio)
            .color(theme_colors.text_main),
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
