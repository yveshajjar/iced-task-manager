use crate::app::AppPage;
use crate::theme::AppTheme;
use crate::theme::ThemeColors;
use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::window::{Id, maximize, *};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use iced::{Element, Subscription, Task, Vector};
use strum::IntoEnumIterator;

use crate::app::AppMessage::AppStart;
use crate::app::{App, AppMessage};
use crate::todo::TodoTitleState::{Editing, Viewing};
use crate::todo::{TodoStatus, TodoTitleState};
use crate::widgets::input_bar::input_bar;
use crate::widgets::sidebar::sidebar;
use crate::widgets::todo_card::todo_card;

use crate::storage;
use crate::todo::TodoFilter;
use crate::todo::Todo;

pub fn settings_page<'a>(app: &'a App) -> iced::Element<'a, AppMessage> {
    let window_ratio = app.window_ratio;
    let current_page = &app.current_page;
    let theme_colors = app.theme.colors();

    let content = column![
        text("Settings")
            .size(28.0 * app.window_ratio)
            .color(theme_colors.text_main),
        text("More options coming soon.")
            .size(15.0 * app.window_ratio)
            .color(theme_colors.text_main),
        row![
            button(text("Light"))
                .on_press(AppMessage::ThemeChanged(AppTheme::Light))
                .style(move |_, status| button_style(
                    app.theme,
                    AppTheme::Light,
                    theme_colors,
                    status,
                    current_page,
                    window_ratio
                )),
            button(text("Dark"))
                .on_press(AppMessage::ThemeChanged(AppTheme::Dark))
                .style(move |_, status| button_style(
                    app.theme,
                    AppTheme::Dark,
                    theme_colors,
                    status,
                    current_page,
                    window_ratio
                ))
        ]
        .spacing(8.0 * window_ratio),
    ]
    .spacing(18.0 * app.window_ratio)
    .align_x(iced::alignment::Horizontal::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

#[inline]
fn button_style(
    current_theme: AppTheme,
    button_theme: AppTheme,
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

    let bg = if current_theme == button_theme {
        selected_bg
    } else {
        inactive_bg
    };

    let border = if current_theme == button_theme {
        selected_border
    } else {
        inactive_border
    };

    let text_color = if current_theme == button_theme {
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
                border
            },
            radius: Radius::new(5.0 * window_ratio),
            ..Default::default()
        },
        text_color: if status == Status::Hovered {
            inactive_text
        } else {
            text_color
        },
        ..Default::default()
    }
}
