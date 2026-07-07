use crate::{todo::Todo, theme::AppTheme};

pub fn load_todos() -> Vec<Todo> {
    let file_content = std::fs::read_to_string("todos.json");
    if file_content.is_err() {
        return Vec::new();
    }

    serde_json::from_str(&file_content.unwrap()).unwrap_or_else(|_| Vec::new())
}

pub fn save_todos(todos: &[Todo]) {
    let file_content = serde_json::to_string_pretty(todos).unwrap();
    std::fs::write("todos.json", file_content).unwrap();
}

pub fn load_theme() -> AppTheme {
    let file_content = std::fs::read_to_string("theme.json");
    if file_content.is_err() {
        return AppTheme::Light;
    }

    serde_json::from_str(&file_content.unwrap()).unwrap_or(AppTheme::Light)
}

pub fn save_theme(theme: AppTheme) {
    let file_content = serde_json::to_string_pretty(&theme).unwrap();
    std::fs::write("theme.json", file_content).unwrap();
}
