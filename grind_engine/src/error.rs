#[derive(Debug)]
pub enum TaskError {
    DBError(rusqlite::Error),
}

impl From<rusqlite::Error> for TaskError {
    fn from(value: rusqlite::Error) -> Self {
        Self::DBError(value)
    }
}
