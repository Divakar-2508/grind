use std::io;

use tauri::ipc::InvokeError;

#[derive(Debug)]
pub enum TaskError {
    InSufficientPermission,
    Mudinchh
}

impl From<rusqlite::Error> for TaskError {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            _ => Self::Mudinchh
        }
    }
}

impl From<io::Error> for TaskError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::PermissionDenied => TaskError::InSufficientPermission,
            _ => Self::Mudinchh
        }
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::InSufficientPermission => write!(f, "InSufficientPermission"),
            TaskError::Mudinchh => write!(f, "Mudinchh")
        }
    }
}

impl std::error::Error for TaskError {}

impl Into<InvokeError> for TaskError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}