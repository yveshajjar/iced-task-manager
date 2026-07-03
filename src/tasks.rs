use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub status: TodoStatus,
    pub title_state: TodoTitleState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum TodoFilter {
    #[default]
    All,
    Active,
    Completed,
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
