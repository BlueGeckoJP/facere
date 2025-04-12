mod sql_manager;

use sql_manager::{SqlManager, SqlTodo};

struct AppState {
    sql_manager: SqlManager,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sql_manager: SqlManager::new().unwrap(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();
    app_state.sql_manager.add_test_todos().unwrap();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_todos, add_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos(state: tauri::State<AppState>) -> Result<Vec<SqlTodo>, String> {
    match state.sql_manager.get_todos() {
        Ok(todos) => Ok(todos),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn add_todo(
    state: tauri::State<AppState>,
    uuid: String,
    title: String,
    checked: bool,
    deadline: String,
) -> Result<(), String> {
    match state.sql_manager.add_todo(SqlTodo {
        uuid,
        title,
        checked,
        deadline,
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
