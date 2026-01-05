mod db;
mod google;

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

#[tauri::command]
fn update_task_order(ordered_ids: Vec<String>, state: State<AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    db.update_task_order(ordered_ids).map_err(|e| e.to_string())
}

// Google Tasks Commands

#[tauri::command]
async fn get_google_user(state: State<'_, AppState>) -> Result<google::GoogleUser, String> {
    let db = {
        let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
        db_guard.as_ref().ok_or("Database not initialized")?.clone()
    };
    google::get_user_profile(&db).await
}

#[tauri::command]
fn get_google_auth_url() -> String {
    google::get_auth_url()
}

#[tauri::command]
async fn finish_google_auth(code: String, state: State<'_, AppState>) -> Result<google::GoogleUser, String> {
    let db = {
        let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
        db_guard.as_ref().ok_or("Database not initialized")?.clone()
    };
    google::exchange_code(&code, &db).await
}

#[tauri::command]
async fn complete_google_auth(state: State<'_, AppState>) -> Result<google::GoogleUser, String> {
    // Start OAuth callback server
    let code = google::start_oauth_server()?;
    
    // Exchange code for tokens
    let db = {
        let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
        db_guard.as_ref().ok_or("Database not initialized")?.clone()
    };
    google::exchange_code(&code, &db).await
}

#[tauri::command]
async fn get_google_task_lists(state: State<'_, AppState>) -> Result<Vec<google::TaskList>, String> {
    let db = {
        let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
        db_guard.as_ref().ok_or("Database not initialized")?.clone()
    };
    google::fetch_task_lists(&db).await
}

#[tauri::command]
async fn import_google_tasks(list_id: String, state: State<'_, AppState>) -> Result<usize, String> {
    let db = {
        let db_guard = state.db.lock().map_err(|_| "Failed to lock mutex")?;
        db_guard.as_ref().ok_or("Database not initialized")?.clone()
    };
    
    let g_tasks = google::fetch_tasks(&db, &list_id).await?;
    let mut count = 0;
    
    for g_task in g_tasks {
        // Map Google Task to local Task
        let task = Task {
            id: g_task.id,
            title: g_task.title,
            completed: g_task.status == "completed",
            priority: "medium".to_string(), // Default, Google doesn't have simple priority like high/low easily accessible without parsing notes or something
            category: "Google Tasks".to_string(), // Or use list title if passed
            due_date: g_task.due.map(|d| d.chars().take(10).collect()), // Simple truncate to YYYY-MM-DD
            position: 0, // Default position, will be sorted to top/bottom depending on logic
        };
        
        // Upsert
        if db.get_task_by_id(&task.id).map_err(|e| e.to_string())?.is_some() {
            db.update_task(task).map_err(|e| e.to_string())?;
        } else {
            db.add_task(task).map_err(|e| e.to_string())?;
        }
        count += 1;
    }
    
    Ok(count)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok(); // Load .env file

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
        .invoke_handler(tauri::generate_handler![
            get_tasks, get_task, add_task, update_task, delete_task, update_task_order,
            get_google_auth_url, finish_google_auth, complete_google_auth, get_google_user, get_google_task_lists, import_google_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}