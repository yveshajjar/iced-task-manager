use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, space, text};
use iced::{Border, Color, Length, Theme};

use crate::app::{App, AppMessage};
use crate::theme::{AppTheme, ThemeColors};
use crate::todo::{Todo, TodoPriority, TodoStatus};

pub fn input_bar<'a>(app: &'a App, todo_input: &str) -> iced::Element<'a, AppMessage> {
    let window_ratio = app.window_ratio;
    let theme_colors = app.theme.colors();

    let input_text = iced::widget::text_input("Enter new todo", &todo_input)
        .on_input(AppMessage::TodoInputChanged)
        .style(move |_, status| input_text_style(theme_colors, status));

    let priority_card = iced::widget::PickList::new(
        [TodoPriority::High, TodoPriority::Medium, TodoPriority::Low],
        Some(app.new_todo_priority),
        AppMessage::TodoPriorityAdded,
    )
    .style(move |_, status| {
        picklist_style(theme_colors, app.new_todo_priority, status, window_ratio)
    })
    .menu_style(move |_| picklist_menu_style(theme_colors))
    .width(Length::Shrink)
    .padding([4.0 * window_ratio, 10.0 * window_ratio]);

    let priority_card_wrapper = container(priority_card).width(Length::Shrink);

    row![
        input_text,
        priority_card_wrapper,
        iced::widget::button("Add").on_press(AppMessage::AddTodo)
    ]
    .spacing(5.0 * window_ratio)
    .width(Length::Fixed(360.0 * window_ratio))
    .height(Length::Fixed(30.0 * window_ratio))
    .into()
}

#[inline]
fn input_text_style(
    theme_colors: ThemeColors,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let bg = theme_colors.input_bg;
    let border = theme_colors.input_border;
    let text = theme_colors.text_main;
    let placeholder = theme_colors.text_placeholder;

    iced::widget::text_input::Style {
        background: bg.into(),
        border: Border {
            radius: Radius::from(5.0),
            color: border,
            ..Default::default()
        },
        placeholder,
        value: text,
        icon: Color::WHITE,
        selection: Color::WHITE,
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

use iced::widget::overlay::menu;

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
