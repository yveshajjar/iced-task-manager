use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::window::{self, Id, maximize, *};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use iced::{Element, Subscription, Task, Vector};
use tracing_subscriber::filter;

use crate::app::AppMessage::AddTodo;
use crate::tasks::TodoTitleState::{Editing, Viewing};
use crate::tasks::{TodoStatus, TodoTitleState};
use crate::widgets::filter_bar::filter_bar;
use crate::widgets::input_bar::input_bar;
use crate::widgets::sidebar::sidebar;
use crate::widgets::todo_card::todo_card;

use super::storage;
use super::tasks::TodoFilter;
use super::tasks::TodoItem;

pub struct App {
    // view: View,

    // Main Window properties
    main_window_id: Option<Id>,
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
    WindowClosed(Id),
    WindowResized(Vector),

    AddTodo,
    TodoInputChanged(String),
    TodoToggled(usize, TodoStatus),

    ShowTodoEdit(usize),
    TodoEditChanged(String),
    EditTodo(usize),
    CancelEditTodo(usize),

    DeleteTodo(usize), // index
    TodoFilterChanged(TodoFilter),

    SettingsPressed,
}

#[derive(Debug)]
pub enum View {}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let (id, open) = window::open(window::Settings::default());
        let max = maximize(id, true);

        (
            Self {
                // view: View {},
                main_window_id: Some(id),
                window_ratio: 1.0,
                window_size: Vector::new(800.0, 600.0),
                todos: storage::load_todos(),
                todo_input_buffer: String::new(),
                todo_filter: TodoFilter::All,
                todo_edit_buffer: String::new(),
                old_todo_title: String::new(),
            },
            // Use batch to run both tasks
            Task::batch([open.map(AppMessage::AppStart), max]),
        )
    }

    pub fn title(&self, _window: iced::window::Id) -> String {
        "Iced Tasks".into()
    }

    fn update(&mut self, msg: AppMessage) -> Task<AppMessage> {
        use AppMessage::*;

        match msg {
            AppStart(id) => {
                println!("Main window opened with ID: {id}");
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
            WindowClosed(id) => {
                // If the main window is closed, exit the application
                if self.main_window_id != Some(id) {
                    println!("Window closed with ID: {id}");
                    return close(id);
                }

                println!("Main window closed with ID: {id}, exiting application");
                iced::exit()
            }
            WindowResized(size) => {
                self.window_ratio = size.x / size.y;
                self.window_size = size;
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
        let close_subscription = iced::window::close_events().map(AppMessage::WindowClosed);

        let window_resize_subscription =
            iced::window::resize_events().map(|(_, size)| AppMessage::WindowResized(size.into()));

        Subscription::batch([close_subscription, window_resize_subscription])
    }

    fn view(&self, _window_id: iced::window::Id) -> Element<'_, AppMessage> {
        use View::*;

        let todos_list: Vec<_> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, todo)| match self.todo_filter {
                TodoFilter::All => true,
                TodoFilter::Active => todo.status == TodoStatus::Active,
                TodoFilter::Completed => todo.status == TodoStatus::Completed,
            })
            .map(|(index, todo)| todo_card(todo, index, self.window_ratio, &self.todo_edit_buffer))
            .collect();

        let todos_liste_len = todos_list.len();

        let todos_column = column(todos_list)
            .spacing(2.0 * self.window_ratio)
            .width(Length::Fixed(320.0 * self.window_ratio))
            .height(Length::Fill);

        let todos_scrollable = iced::widget::scrollable(todos_column)
            .width(Length::Fixed(370.0 * self.window_ratio))
            .height(Length::Fixed(460.0 * self.window_ratio))
            .style(scrollable_style);

        let filter_bar = filter_bar(self.window_ratio, &self.todo_filter);

        let input_bar = input_bar(&self.todo_input_buffer, self.window_ratio);

        let empty_text = match self.todo_filter {
            TodoFilter::All => "No todos yet. Add your first task above.",
            TodoFilter::Active => "No active todos.",
            TodoFilter::Completed => "No completed todos yet.",
        };

        let empty_todos_list_text = text(empty_text)
            .size(15.0 * self.window_ratio)
            .color(Color::from_rgb8(148, 163, 184));

        let what_to_display = if todos_liste_len != 0 {
            container(todos_scrollable)
        } else {
            container(empty_todos_list_text)
        };

        let main_content = container(
            column![
                text("My Tasks")
                    .size(28.0 * self.window_ratio)
                    .color(Color::from_rgb8(30, 41, 59)),
                input_bar,
                what_to_display
            ]
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .spacing(18.0 * self.window_ratio),
        )
        .width(560.0 * self.window_ratio)
        .height(400.0 * self.window_ratio);

        let main_content_wrapper = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let sidebar = sidebar(self.window_ratio, self.todo_filter);

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
        iced::daemon(Self::new, Self::update, Self::view)
            .title(Self::title)
            .subscription(Self::subscription)
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
