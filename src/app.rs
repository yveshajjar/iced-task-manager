use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::window::{self, Id, maximize, *};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use iced::{Element, Subscription, Task, Vector};
use strum::IntoEnumIterator;

use crate::app::AppMessage::AppStart;
use crate::tasks::TodoTitleState::{Editing, Viewing};
use crate::tasks::{TodoStatus, TodoTitleState};
use crate::widgets::input_bar::input_bar;
use crate::widgets::sidebar::sidebar;
use crate::widgets::todo_card::todo_card;

use super::storage;
use super::tasks::TodoFilter;
use super::tasks::TodoItem;

pub struct App {
    // Window properties
    window_ratio: f32,
    window_size: Vector,

    // Task properties
    todos: Vec<TodoItem>,
    todo_input_buffer: String,
    todo_edit_buffer: String,
    old_todo_title: String,

    // Filter properties
    todo_filter: TodoFilter,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    // Start the app and open the main window
    AppStart(Id),
    WindowResized(Vector),

    AddTodo,
    TodoInputChanged(String),
    TodoToggled(usize, TodoStatus),

    ShowTodoEdit(usize),
    TodoEditChanged(String),
    EditTodo(usize),
    CancelEditTodo(usize),

    DeleteTodo(usize),
    ClearCompletedTodos,
    TodoFilterChanged(TodoFilter),

    SettingsPressed,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window_ratio: 1.0,
            window_size: Vector::new(800.0, 600.0),
            todos: storage::load_todos(),
            todo_input_buffer: String::new(),
            todo_filter: TodoFilter::All,
            todo_edit_buffer: String::new(),
            old_todo_title: String::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(&self) -> String {
        "Iced Tasks".into()
    }

    fn update(&mut self, msg: AppMessage) -> Task<AppMessage> {
        use AppMessage::*;

        match msg {
            AppStart(id) => iced::window::maximize(id, true),
            WindowResized(size) => {
                self.window_ratio = size.x / size.y;
                self.window_size = size;
                Task::none()
            }
            AddTodo => {
                if self.todo_input_buffer.trim().is_empty() {
                    return Task::none();
                }

                self.todos.push(TodoItem {
                    title: self.todo_input_buffer.clone(),
                    status: TodoStatus::Active,
                    title_state: Viewing,
                });
                self.todo_input_buffer.clear();

                storage::save_todos(&self.todos);

                Task::none()
            }
            TodoInputChanged(input) => {
                self.todo_input_buffer = input;
                Task::none()
            }
            TodoToggled(index, status) => {
                let todo = &mut self.todos[index];
                todo.status = status;

                storage::save_todos(&self.todos);

                Task::none()
            }
            ShowTodoEdit(index) => {
                let todo = &mut self.todos[index];

                self.old_todo_title = todo.title.clone();
                todo.title_state = TodoTitleState::Editing;

                Task::none()
            }
            TodoEditChanged(title) => {
                self.todo_edit_buffer = title;

                Task::none()
            }
            EditTodo(index) => {
                let todo = &mut self.todos[index];

                todo.title = self.todo_edit_buffer.clone();

                self.todo_edit_buffer.clear();
                self.old_todo_title.clear();
                todo.title_state = TodoTitleState::Viewing;

                storage::save_todos(&self.todos);

                Task::none()
            }
            CancelEditTodo(usize) => {
                let todo = &mut self.todos[usize];

                todo.title = self.old_todo_title.clone();
                self.todo_edit_buffer.clear();
                self.old_todo_title.clear();
                todo.title_state = TodoTitleState::Viewing;

                Task::none()
            }
            DeleteTodo(index) => {
                self.todos.remove(index);

                storage::save_todos(&self.todos);

                Task::none()
            }
            ClearCompletedTodos => {
                self.todos.retain(|todo| todo.status == TodoStatus::Active);
                Task::none()
            }
            TodoFilterChanged(filter) => {
                self.todo_filter = filter;
                Task::none()
            }
            SettingsPressed => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        let window_open = iced::window::open_events().map(AppStart);

        let window_resize =
            iced::window::resize_events().map(|(_, size)| AppMessage::WindowResized(size.into()));

        Subscription::batch([window_open, window_resize])
    }

    fn view(&self) -> Element<'_, AppMessage> {
        let todos_count: Vec<usize> = TodoFilter::iter()
            .map(|filter| {
                self.todos
                    .iter()
                    .filter(|todo| filter.matches(&todo.status))
                    .count()
            })
            .collect();

        let todos_list: Vec<_> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, todo)| self.todo_filter.matches(&todo.status))
            .map(|(index, todo)| todo_card(todo, index, self.window_ratio, &self.todo_edit_buffer))
            .collect();

        let sidebar = sidebar(self.window_ratio, self.todo_filter, todos_count);

        let has_todos = !todos_list.is_empty();

        let todos_column = column(todos_list)
            .spacing(2.0 * self.window_ratio)
            .width(Length::Fixed(320.0 * self.window_ratio))
            .height(Length::Fill);

        let todos_scrollable = iced::widget::scrollable(todos_column)
            .width(Length::Fixed(370.0 * self.window_ratio))
            .height(Length::Fixed(460.0 * self.window_ratio))
            .style(scrollable_style);

        let input_bar = input_bar(&self.todo_input_buffer, self.window_ratio);

        let empty_text = match self.todo_filter {
            TodoFilter::All => "No todos yet. Add your first task above.",
            TodoFilter::Active => "No active todos.",
            TodoFilter::Completed => "No completed todos yet.",
        };

        let empty_state_text = text(empty_text)
            .size(15.0 * self.window_ratio)
            .color(Color::from_rgb8(148, 163, 184));

        let todos_content = if has_todos {
            container(todos_scrollable)
        } else {
            container(empty_state_text)
        };

        let main_content_elements = column![
            text("My Tasks")
                .size(28.0 * self.window_ratio)
                .color(Color::from_rgb8(30, 41, 59)),
            input_bar,
            todos_content,
        ]
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .spacing(18.0 * self.window_ratio);

        let main_content = container(main_content_elements)
            .width(560.0 * self.window_ratio)
            .height(400.0 * self.window_ratio);

        let main_content_wrapper = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let layout = row![sidebar, main_content_wrapper]
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Color::WHITE.into()),
                ..Default::default()
            })
            .into()
    }

    pub fn run() -> iced::Result {
        iced::application(Self::new, Self::update, Self::view)
            .subscription(Self::subscription)
            .title(Self::title)
            .antialiasing(true)
            .run()
    }
}

#[inline]
fn scrollable_style(
    theme: &Theme,
    status: iced::widget::scrollable::Status,
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
