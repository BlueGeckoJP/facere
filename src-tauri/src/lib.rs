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
        .invoke_handler(tauri::generate_handler![get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos(state: tauri::State<AppState>) -> Vec<SqlTodo> {
    state.sql_manager.get_todos().unwrap()
}
