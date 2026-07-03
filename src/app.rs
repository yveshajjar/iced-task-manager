use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced::widget::{button, column, container, row, space, text};
use iced::window::{self, Id, maximize, *};
use iced::{Border, Color, Length, Shadow, Theme, theme};
use iced::{Element, Subscription, Task, Vector};
use tracing_subscriber::filter;

use crate::app::AppMessage::AddTodo;
use crate::tasks::TodoStatus;
use crate::widgets::filter_bar::filter_bar;
use crate::widgets::input_bar::input_bar;
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
    todo_input: String,

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
    TodoToggled(usize, TodoStatus), // (index, completed)
    DeleteTodo(usize),              // index
    TodoFilterChanged(TodoFilter),
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
                todo_input: String::new(),
                todo_filter: TodoFilter::All,
            },
            // Use batch to run both tasks
            Task::batch([open.map(AppMessage::AppStart), max]),
        )
    }

    pub fn title(&self, _window: iced::window::Id) -> String {
        "Task Manager".into()
    }

    fn update(&mut self, msg: AppMessage) -> Task<AppMessage> {
        use AppMessage::*;

        match msg {
            AppStart(id) => {
                println!("Main window opened with ID: {id}");
                Task::none()
            }
            AddTodo => {
                if self.todo_input.trim().is_empty() {
                    return Task::none();
                }

                self.todos.push(TodoItem {
                    title: self.todo_input.clone(),
                    status: TodoStatus::Active,
                });
                self.todo_input.clear();

                storage::save_todos(&self.todos);

                Task::none()
            }
            TodoInputChanged(input) => {
                self.todo_input = input;
                Task::none()
            }
            TodoToggled(index, status) => {
                let todo = &mut self.todos[index];
                todo.status = status;

                storage::save_todos(&self.todos);

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

        let todos_list = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, todo)| match self.todo_filter {
                TodoFilter::All => true,
                TodoFilter::Active => todo.status == TodoStatus::Active,
                TodoFilter::Completed => todo.status == TodoStatus::Completed,
            })
            .map(|(index, todo)| todo_card(todo, index, self.window_ratio));

        let todos_cards = column(todos_list)
            .spacing(2.0 * self.window_ratio)
            .width(Length::Fixed(320.0 * self.window_ratio))
            .height(Length::Fill);

        let scrollable = iced::widget::scrollable(todos_cards)
            .width(Length::Fixed(370.0 * self.window_ratio))
            .height(Length::Fixed(460.0 * self.window_ratio))
            .style(scrollable_style);

        let filter_bar = filter_bar(self.window_ratio);

        let input_bar = input_bar(&self.todo_input, self.window_ratio);

        let todos_card = container(
            column![input_bar, filter_bar, scrollable]
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(18.0 * self.window_ratio),
        )
        .width(400.0 * self.window_ratio)
        .height(400.0 * self.window_ratio);

        container(todos_card)
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
