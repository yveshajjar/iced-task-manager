use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{Column, Space, button, column, container, row, space, text};
use iced::{Border, Color, Element, Length};
use strum::IntoEnumIterator;

use crate::app::{App, AppMessage, AppPage};
use crate::todo::{TodoFilter, Todo, TodoStatus, TodoTitleState};
use crate::theme::{AppTheme, ThemeColors};

pub fn sidebar<'a>(app: &'a App, todos_count: Vec<usize>) -> Element<'a, AppMessage> {
    let window_ratio = app.window_ratio;
    let current_page = &app.current_page;
    let theme_colors = app.theme.colors();

    let filter_buttons: Vec<_> = TodoFilter::iter()
        .zip(&todos_count)
        .map(move |(filter, count)| {
            sidebar_filter_button(
                theme_colors,
                filter.to_string(),
                current_page,
                filter,
                *count,
                window_ratio,
            )
        })
        .collect();

    let clear_button = button(container(text("Clear completed")).center(Length::Fill))
        .on_press(AppMessage::ClearCompletedTodos)
        .width(Length::Fill)
        .height(36.0 * window_ratio)
        .padding(0)
        .style(move |_, status| clear_button_style(theme_colors, status, window_ratio));

    let settings_button = button(text("Settings"))
        .on_press(AppMessage::PageChanged(AppPage::Settings))
        .style(move |_, status| {
            settings_button_style(theme_colors, status, current_page, window_ratio)
        });

    let mut content = Column::new()
        .push(
            text("Iced Tasks")
                .size(22.0 * window_ratio)
                .color(theme_colors.text_main),
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
            background: Some(theme_colors.sidebar_bg.into()),
            border: Border {
                width: 1.0 * window_ratio,
                color: theme_colors.sidebar_border,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

#[inline]
fn sidebar_filter_button<'a>(
    theme_colors: ThemeColors,
    button_text: String,
    current_page: &'a AppPage,
    filter: TodoFilter,
    count: usize,
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
        .style(move |_, status| {
            button_style(theme_colors, filter, current_page, status, window_ratio)
        })
        .on_press(AppMessage::TodoFilterChanged(filter))
        .into()
}

#[inline]
fn clear_button_style(
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

#[inline]
fn button_style(
    theme_colors: ThemeColors,
    button_filter: TodoFilter,
    current_page: &AppPage,
    status: Status,
    window_ratio: f32,
) -> Style {
    // Selected filter
    let selected_bg = theme_colors.blue_bg;
    let selected_border = theme_colors.blue_border;
    let selected_text = theme_colors.blue_text;

    // Inactive filter
    let inactive_bg = theme_colors.inactive_bg;
    let inactive_border = theme_colors.inactive_border;
    let inactive_text = theme_colors.inactive_text;

    // Hover inactive
    let inactive_bg_hover = theme_colors.inactive_bg_hover;
    let inactive_border_hover = theme_colors.inactive_border_hover;

    let bg = match current_page {
        AppPage::Tasks(filter) => {
            if *filter == button_filter {
                selected_bg
            } else {
                inactive_bg
            }
        }
        AppPage::Settings => inactive_bg,
    };

    let border_color = match current_page {
        AppPage::Tasks(filter) => {
            if *filter == button_filter {
                selected_border
            } else {
                inactive_border
            }
        }
        AppPage::Settings => inactive_border,
    };

    let text = match current_page {
        AppPage::Tasks(filter) => {
            if *filter == button_filter {
                selected_text
            } else {
                inactive_text
            }
        }
        AppPage::Settings => inactive_text,
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
fn settings_button_style(
    theme_colors: ThemeColors,
    status: iced::widget::button::Status,
    current_page: &AppPage,
    window_ratio: f32,
) -> Style {
    use iced::widget::button::Status;

    // Selected filter
    let selected_bg = theme_colors.blue_bg;
    let selected_border = theme_colors.blue_border;
    let selected_text = theme_colors.blue_text;

    // Inactive filter
    let inactive_bg = theme_colors.inactive_bg;
    let inactive_border = theme_colors.inactive_border;
    let inactive_text = theme_colors.inactive_text;

    // Hover inactive
    let inactive_bg_hover = theme_colors.inactive_bg_hover;
    let inactive_border_hover = theme_colors.inactive_border_hover;

    let bg = if *current_page == AppPage::Settings {
        selected_bg
    } else {
        inactive_bg
    };

    let border_color = if *current_page == AppPage::Settings {
        selected_border
    } else {
        inactive_border
    };

    let text_color = if *current_page == AppPage::Settings {
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
        text_color,
        ..Default::default()
    }
}
