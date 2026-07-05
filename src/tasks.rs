use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub status: TodoStatus,
    pub title_state: TodoTitleState,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum TodoFilter {
    #[default]
    All,
    Active,
    Completed,
}

impl TodoFilter {
    pub fn matches(&self, status: &TodoStatus) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => *status == TodoStatus::Active,
            TodoFilter::Completed => *status == TodoStatus::Completed,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TodoTitleState {
    Viewing,
    Editing,
}
