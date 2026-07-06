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
use crate::tasks::TodoTitleState::{Editing, Viewing};
use crate::tasks::{TodoStatus, TodoTitleState};
use crate::widgets::input_bar::input_bar;
use crate::widgets::sidebar::sidebar;
use crate::widgets::todo_card::todo_card;

use crate::storage;
use crate::tasks::TodoFilter;
use crate::tasks::TodoItem;

pub fn settings_page<'a>(app: &'a App) -> iced::Element<'a, AppMessage> {
    let content = column![
        text("Settings")
            .size(28.0 * app.window_ratio)
            .color(Color::from_rgb8(30, 41, 59)),
        text("More options coming soon.")
            .size(15.0 * app.window_ratio)
            .color(Color::from_rgb8(148, 163, 184)),
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
