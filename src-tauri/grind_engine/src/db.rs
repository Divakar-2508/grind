use crate::error::TaskError;
use crate::models::Task;
use std::path::Path;

use rusqlite::{params, Connection};

pub struct TaskBase {
    conn: Connection,
}

impl TaskBase {
    const CREATE_TASK: &str = r#"
        INSERT INTO tasks(name, done, command)
        VALUES(?1, ?2, ?3) RETURNING ID 
    "#;

    const RETRIEVE_TASKS: &str = r#"
        SELECT id, name, done, command
        FROM tasks
    "#;

    const DELETE_TASK: &str = r#"
        DELETE FROM tasks
        WHERE id = (?1)
    "#;

    const MARK_TASK_DONE: &str = r#"
        UPDATE tasks
        SET done = true
        WHERE id = (?1)
    "#;

    const TASKS_DONE_COUNT: &str = r#"
        SELECT COUNT(*) from tasks 
        WHERE done = true
    "#;

    const RESET_TASKS: &str = r#"
        UPDATE tasks
        SET done = false
    "#;

    const GET_STREAKS_COUNT: &str = r#"
        SELECT streaks
        FROM admin
    "#;

    const INCREMENT_STREAKS: &str = r#"
        UPDATE admin
        SET streaks = streaks + 1
    "#;

    const RESET_STREAKS: &str = r#"
        UPDATE admin
        SET streaks = 0
    "#;

    const LAST_LOGIN_DATE: &str = r#"
        SELECT last_login 
        FROM admin
        LIMIT 1
    "#;

    const UPDATE_LOGIN_DATE: &str = r#"
        UPDATE admin
        SET last_login = date() 
    "#;

    pub fn new(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;

        let queries = include_str!("schema.sql");
        conn.execute_batch(queries)?;

        Ok(Self { conn })
    }

    pub fn create_task(&self, task: &mut Task) -> Result<(), TaskError> {
        let mut statement = self.conn.prepare(Self::CREATE_TASK)?;
        let task_id = statement.query_row(params![task.name, task.done, task.command], |row| {
            row.get::<_, u32>(0) //id
        })?;

        task.id = task_id;

        Ok(())
    }

    pub fn retrieve_tasks(&self) -> Result<Vec<Task>, TaskError> {
        let mut stmt = self.conn.prepare(Self::RETRIEVE_TASKS)?;
        let query_result = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get("id")?,
                name: row.get("name")?,
                done: row.get("done")?,
                command: row.get("command")?,
            })
        })?;

        query_result
            .collect::<Result<Vec<Task>, _>>()
            .map_err(TaskError::from)
    }

    pub fn delete_task(&self, task_id: u32) -> Result<(), TaskError> {
        Ok(self
            .conn
            .execute(Self::DELETE_TASK, [task_id])
            .map(|_| ())?)
    }

    pub fn mark_task_done(&self, task_id: u32) -> Result<(), TaskError> {
        self.conn
            .execute(Self::MARK_TASK_DONE, [task_id])
            .map(|_| ())
            .map_err(TaskError::from)
    }

    pub fn task_done_count(&self) -> Result<u32, TaskError> {
        self.conn
            .query_row(Self::TASKS_DONE_COUNT, [], |row| row.get::<_, u32>(0))
            .map_err(TaskError::from)
    }

    pub fn reset_tasks(&self) -> Result<(), TaskError> {
        self.conn
            .execute(Self::RESET_TASKS, [])
            .map(|_| ())
            .map_err(TaskError::from)
    }

    pub fn get_streaks_count(&self) -> Result<u32, TaskError> {
        self.conn
            .query_row(Self::GET_STREAKS_COUNT, [], |row| {
                row.get::<_, u32>("streaks")
            })
            .map_err(TaskError::from)
    }

    pub fn increment_streaks(&self) -> Result<(), TaskError> {
        self.conn
            .execute(Self::INCREMENT_STREAKS, [])
            .map(|_| ())
            .map_err(TaskError::from)
    }

    pub fn reset_streaks(&self) -> Result<(), TaskError> {
        self.conn
            .execute(Self::RESET_STREAKS, [])
            .map(|_| ())
            .map_err(TaskError::from)
    }

    pub fn last_login_date(&self) -> Result<chrono::NaiveDate, TaskError> {
        self.conn
            .query_row(Self::LAST_LOGIN_DATE, [], |row| {
                row.get::<_, chrono::NaiveDate>("last_login")
            })
            .map_err(TaskError::from)
    }

    pub fn update_login_date(&self) -> Result<(), TaskError> {
        self.conn
            .execute(Self::UPDATE_LOGIN_DATE, [])
            .map(|_| ())
            .map_err(TaskError::from)
    }
}
