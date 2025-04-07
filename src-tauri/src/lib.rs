use sql_manager::SqlManager;

mod sql_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sql_mgr = SqlManager::new().expect("failed to create SqlManager");
    sql_mgr.add_test_todos().expect("failed to add test todos");

    let todos = sql_mgr.get_todos().expect("failed to get todos");

    for todo in todos {
        println!(
            "uuid: {}, title: {}, checked: {}, deadline: {}",
            todo.uuid, todo.title, todo.checked, todo.deadline
        );
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
