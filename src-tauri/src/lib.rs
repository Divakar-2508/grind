mod models;
mod db;
mod error;

use db::TaskBase;
use error::TaskError;
use models::Task;
use tauri::{Manager, State};

#[tauri::command]
fn get_tasks(db: State<TaskBase>) -> Result<Vec<Task>, TaskError> {
    let tasks = db.get_entries()?;
    Ok(tasks)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let task_base = TaskBase::init_db().unwrap();
            app.manage(task_base);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .unwrap();
}