use iced::Length;
use iced::widget::{container, row};
use iced::window::Id;
use iced::{Element, Subscription, Task, Vector};
use strum::IntoEnumIterator;

use crate::app::AppMessage::AppStart;
use crate::pages::settings::settings_page;
use crate::pages::todos::todos_page;
use crate::storage::{load_theme, save_theme};
use crate::theme::AppTheme;
use crate::todo::TodoTitleState::Viewing;
use crate::todo::{TodoMessage, TodoPriority, TodoSort, TodoStatus, TodoTitleState};
use crate::widgets::sidebar::sidebar;

use super::storage;
use super::todo::Todo;
use super::todo::TodoFilter;

pub struct App {
    // App pages
    pub current_page: AppPage,

    // App theme
    pub theme: AppTheme,

    // Window properties
    pub window_ratio: f32,
    pub window_size: Vector,

    // Task properties
    pub todos: Vec<Todo>,
    pub todo_input_buffer: String,
    pub todo_edit_buffer: String,
    pub old_todo_title: String,

    pub new_todo_priority: TodoPriority,

    pub todo_sort: TodoSort,

    pub todo_search_buffer: String,
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

    Todo(TodoMessage),
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: AppPage::Tasks(TodoFilter::All),
            theme: load_theme(),
            window_ratio: 1.0,
            window_size: Vector::new(800.0, 600.0),
            todos: storage::load_todos(),
            todo_input_buffer: String::new(),
            todo_edit_buffer: String::new(),
            old_todo_title: String::new(),
            new_todo_priority: TodoPriority::Medium,
            todo_sort: TodoSort::Created,
            todo_search_buffer: String::new(),
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

                save_theme(theme);

                Task::none()
            }
            Todo(todo_msg) => self.update_todo(todo_msg),
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
            AppPage::Tasks(filter) => todos_page(self, filter),
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

impl App {
    fn update_todo(&mut self, todo_msg: TodoMessage) -> Task<AppMessage> {
        use TodoMessage::*;

        match todo_msg {
            Add => {
                if self.todo_input_buffer.trim().is_empty() {
                    return Task::none();
                }

                self.todos.push(Todo {
                    title: self.todo_input_buffer.clone(),
                    title_state: Viewing,
                    status: TodoStatus::Active,
                    priority: self.new_todo_priority,
                });

                self.todo_input_buffer.clear();
                self.new_todo_priority = TodoPriority::Medium;

                storage::save_todos(&self.todos);

                Task::none()
            }
            InputChanged(input) => {
                self.todo_input_buffer = input;
                Task::none()
            }
            Toggled(index, status) => {
                let todo = &mut self.todos[index];
                todo.status = status;

                storage::save_todos(&self.todos);

                Task::none()
            }
            ShowEdit(index) => {
                let todo = &mut self.todos[index];

                self.old_todo_title = todo.title.clone();
                todo.title_state = TodoTitleState::Editing;

                Task::none()
            }
            EditChanged(title) => {
                self.todo_edit_buffer = title;

                Task::none()
            }
            Edit(index) => {
                let todo = &mut self.todos[index];

                todo.title = self.todo_edit_buffer.clone();

                self.todo_edit_buffer.clear();
                self.old_todo_title.clear();
                todo.title_state = TodoTitleState::Viewing;

                storage::save_todos(&self.todos);

                Task::none()
            }
            CancelEdit(usize) => {
                let todo = &mut self.todos[usize];

                todo.title = self.old_todo_title.clone();
                self.todo_edit_buffer.clear();
                self.old_todo_title.clear();
                todo.title_state = TodoTitleState::Viewing;

                Task::none()
            }
            Delete(index) => {
                self.todos.remove(index);

                storage::save_todos(&self.todos);

                Task::none()
            }
            ClearCompleted => {
                self.todos.retain(|todo| todo.status == TodoStatus::Active);
                Task::none()
            }
            FilterChanged(filter) => {
                self.current_page = AppPage::Tasks(filter);

                Task::none()
            }
            PriorityChanged(index, priority) => {
                let todo = &mut self.todos[index];

                todo.priority = priority;
                storage::save_todos(&self.todos);

                Task::none()
            }
            NewPriorityChanged(priority) => {
                self.new_todo_priority = priority;

                Task::none()
            }
            SortChanged(sort) => {
                self.todo_sort = sort;

                Task::none()
            }
            SearchChanged(search) => {
                self.todo_search_buffer = search;
                Task::none()
            }
        }
    }
}
