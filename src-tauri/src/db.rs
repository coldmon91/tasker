use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub priority: String,
    pub category: String,
    pub due_date: Option<String>,
    pub position: i32,
}

#[derive(Clone)]
pub struct Database {
    path: String,
}

impl Database {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        use tauri::Manager;
        let app_dir = app_handle.path().app_data_dir().unwrap();
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir).unwrap();
        }
        let path = app_dir.join("tasks.db").to_str().unwrap().to_string();
        Self { path }
    }

    pub fn init(&self) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL,
                priority TEXT NOT NULL,
                category TEXT NOT NULL,
                due_date TEXT
            )",
            [],
        )?;
        
        // Attempt to add position column if it doesn't exist
        // We ignore the error if the column already exists
        let _ = conn.execute("ALTER TABLE tasks ADD COLUMN position INTEGER DEFAULT 0", []);

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT id, title, completed, priority, category, due_date, position FROM tasks ORDER BY position ASC")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
                priority: row.get(3)?,
                category: row.get(4)?,
                due_date: row.get(5)?,
                position: row.get(6).unwrap_or(0),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn get_task_by_id(&self, id: &str) -> Result<Option<Task>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT id, title, completed, priority, category, due_date, position FROM tasks WHERE id = ?1")?;
        let mut task_iter = stmt.query_map(params![id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
                priority: row.get(3)?,
                category: row.get(4)?,
                due_date: row.get(5)?,
                position: row.get(6).unwrap_or(0),
            })
        })?;

        if let Some(task) = task_iter.next() {
            return Ok(Some(task?));
        }
        Ok(None)
    }

    pub fn add_task(&self, task: Task) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        
        // Get max position to append to the end
        let max_pos: i32 = conn.query_row(
            "SELECT COALESCE(MAX(position), -1) FROM tasks",
            [],
            |row| row.get(0),
        ).unwrap_or(-1);

        conn.execute(
            "INSERT INTO tasks (id, title, completed, priority, category, due_date, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task.id, 
                task.title, 
                task.completed, 
                task.priority, 
                task.category, 
                task.due_date,
                max_pos + 1
            ],
        )?;
        Ok(())
    }

    pub fn update_task(&self, task: Task) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "UPDATE tasks SET title = ?2, completed = ?3, priority = ?4, category = ?5, due_date = ?6, position = ?7 WHERE id = ?1",
            params![
                task.id, 
                task.title, 
                task.completed, 
                task.priority, 
                task.category, 
                task.due_date,
                task.position
            ],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: &str) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn update_task_order(&self, ordered_ids: Vec<String>) -> Result<()> {
        let mut conn = Connection::open(&self.path)?;
        let tx = conn.transaction()?;
        
        for (index, id) in ordered_ids.iter().enumerate() {
            tx.execute(
                "UPDATE tasks SET position = ?1 WHERE id = ?2",
                params![index as i32, id],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
}
