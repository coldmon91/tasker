mod db;

use db::{Database, Task};
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Mutex<Option<Database>>,
}

#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.get_tasks().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_task(id: String, state: State<AppState>) -> Result<Option<Task>, String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.get_task_by_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_task(task: Task, state: State<AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.add_task(task).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_task(task: Task, state: State<AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.update_task(task).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_task(id: String, state: State<AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.delete_task(&id).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            let db = Database::new(app.handle());
            db.init().expect("Failed to initialize database");
            
            let state = app.state::<AppState>();
            *state.db.lock().unwrap() = Some(db);
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_tasks, get_task, add_task, update_task, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}