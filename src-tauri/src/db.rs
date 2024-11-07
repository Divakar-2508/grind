use std::{fs, sync::Mutex};
use directories::ProjectDirs;
use rusqlite::Connection;
use crate::{error::TaskError, models::Task};

pub struct TaskBase {
    pub conn: Mutex<Connection>,
}

impl TaskBase {
    pub fn init_db() -> Result<TaskBase, TaskError> {
        let project_dir = ProjectDirs::from("", "blaze", "daily_byte").unwrap();
        let mut data_path = project_dir.data_dir().to_path_buf();

        if !fs::exists(&data_path).unwrap() {
            fs::create_dir_all(&data_path)?;
        }

        data_path.push("taskbase.db");
        let conn = Connection::open(data_path)?;
        
        conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            repeat BOOLEAN NOT NULL,
            created_time TEXT NOT NULL,
            desc TEXT,
            link TEXT
        )", [])?;

        Ok(TaskBase {
            conn: Mutex::new(conn)
        })
    }

    pub fn get_entries(&self) -> Result<Vec<Task>, TaskError> {
        self.clean_entries()?;
        let conn = self.conn.lock().unwrap();
        let mut get_statement = conn.prepare("
            select id, title, desc, link from tasks 
        ")?;

        let tasks: Vec<Task> = get_statement.query_and_then([], |row| -> Result<Task, rusqlite::Error> {
            let id: u32 = row.get("id")?;
            let title: String = row.get("title")?;
            let desc: Option<String> = row.get("desc")?;
            let link: Option<String> = row.get("link")?;

            Ok(Task::new(id, title, desc, link))
        })?
        .filter_map(|task_res| task_res.ok())
        .collect();

        Ok(tasks)
    }

    pub fn clean_entries(&self) -> Result<(), TaskError> {
        let conn = self.conn.lock().unwrap();

        conn.execute("
            DELETE FROM tasks
            WHERE julianday('now') - julianday(created_time) > 1
            AND repeat = false
        ", []).map(|_| ()).map_err(TaskError::from)
    }
}
