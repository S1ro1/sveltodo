use serde;

#[derive(serde::Deserialize)]
pub struct RequestTask {
    pub title: String,
    pub description: String,
    pub difficulty: i32,
}

#[derive(serde::Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: String,
    difficulty: i32,
    finished: bool,
}

impl ResponseTask {
    pub fn new(
        id: i32,
        title: String,
        description: String,
        difficulty: i32,
        finished: bool,
    ) -> Self {
        Self {
            id,
            title,
            description,
            difficulty,
            finished,
        }
    }
}
