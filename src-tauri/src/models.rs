use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    id: u32,
    title: String,
    desc: Option<String>,
    link: Option<String>,
}

impl Task {
    pub fn new(id: u32, title: String, desc: Option<String>, link: Option<String>) -> Self {
        Self {
            id,
            title,
            desc,
            link
        }
    }
}
