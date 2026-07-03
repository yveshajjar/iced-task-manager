use crate::tasks::TodoItem;

pub fn load_todos() -> Vec<TodoItem> {
    let file_content = std::fs::read_to_string("todos.json");
    if file_content.is_err() {
        return Vec::new();
    }

    serde_json::from_str(&file_content.unwrap()).unwrap_or_else(|_| Vec::new())
}

pub fn save_todos(todos: &[TodoItem]) {
    let file_content = serde_json::to_string_pretty(todos).unwrap();
    std::fs::write("todos.json", file_content).unwrap();
}
