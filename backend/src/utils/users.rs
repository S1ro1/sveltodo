use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUserRegister {
    pub username: String,
    pub password: String,
    pub repeated_password: String,
}

#[derive(Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct ResponseUser {
    id: i32,
    name: String,
}

impl ResponseUser {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
