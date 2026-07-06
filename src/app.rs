use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::window::{self, Id, maximize, *};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use iced::{Element, Subscription, Task, Vector};
use strum::IntoEnumIterator;

use crate::app::AppMessage::AppStart;
use crate::pages::settings::settings_page;
use crate::pages::tasks::tasks_page;
use crate::tasks::TodoTitleState::{Editing, Viewing};
use crate::tasks::{TodoStatus, TodoTitleState};
use crate::theme::{AppTheme, ThemeColors};
use crate::widgets::input_bar::input_bar;
use crate::widgets::sidebar::sidebar;
use crate::widgets::todo_card::todo_card;

use super::storage;
use super::tasks::TodoFilter;
use super::tasks::TodoItem;

pub struct App {
    // App pages
    pub current_page: AppPage,

    // App theme
    pub theme: AppTheme,

    // Window properties
    pub window_ratio: f32,
    pub window_size: Vector,

    // Task properties
    pub todos: Vec<TodoItem>,
    pub todo_input_buffer: String,
    pub todo_edit_buffer: String,
    pub old_todo_title: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPage {
    Tasks(TodoFilter),
    Settings,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    // App related messages
    AppStart(Id),
    WindowResized(Vector),
    PageChanged(AppPage),
    ThemeChanged(AppTheme),

    // Todo related messages
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: AppPage::Tasks(TodoFilter::All),
            theme: AppTheme::Light,
            window_ratio: 1.0,
            window_size: Vector::new(800.0, 600.0),
            todos: storage::load_todos(),
            todo_input_buffer: String::new(),
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
            PageChanged(page) => {
                self.current_page = page;
                Task::none()
            }
            ThemeChanged(theme) => {
                self.theme = theme;

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
                self.current_page = AppPage::Tasks(filter);

                Task::none()
            }
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

        let theme_colors = self.theme.colors();

        let sidebar = sidebar(self, todos_count);

        let current_page = match self.current_page {
            AppPage::Tasks(filter) => tasks_page(self, filter),
            AppPage::Settings => settings_page(self),
        };

        let current_page = container(current_page)
            .width(560.0 * self.window_ratio)
            .height(400.0 * self.window_ratio);

        let current_page_wrapper = container(current_page)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let main_layout = row![sidebar, current_page_wrapper]
            .width(Length::Fill)
            .height(Length::Fill);

        container(main_layout)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(theme_colors.app_bg.into()),
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
