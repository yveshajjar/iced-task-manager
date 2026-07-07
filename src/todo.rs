use iced::Color;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::theme::ThemeColors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub title_state: TodoTitleState,
    pub status: TodoStatus,
    #[serde(default)]
    pub priority: TodoPriority,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum TodoStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TodoTitleState {
    Viewing,
    Editing,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Default, PartialOrd, Ord,
)]
pub enum TodoPriority {
    Low,
    #[default]
    Medium,
    High,
}

impl TodoPriority {
    pub fn colors(self, colors: ThemeColors) -> (Color, Color, Color) {
        match self {
            TodoPriority::Low => (
                colors.priority_low_bg,
                colors.priority_low_text,
                colors.priority_low_border,
            ),
            TodoPriority::Medium => (
                colors.priority_medium_bg,
                colors.priority_medium_text,
                colors.priority_medium_border,
            ),
            TodoPriority::High => (
                colors.priority_high_bg,
                colors.priority_high_text,
                colors.priority_high_border,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, EnumIter)]
pub enum TodoSort {
    #[default]
    Created,
    PriorityHighFirst,
    PriorityLowFirst,
    CompletedLast,
}

impl std::fmt::Display for TodoSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoSort::Created => write!(f, "Created"),
            TodoSort::PriorityHighFirst => write!(f, "Priority: High first"),
            TodoSort::PriorityLowFirst => write!(f, "Priority: Low first"),
            TodoSort::CompletedLast => write!(f, "Completed last"),
        }
    }
}
