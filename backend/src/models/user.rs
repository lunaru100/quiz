use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new_guest(id: Uuid) -> Self {
        Self {
            id,
            username: "guest".to_string(),
            email: String::new(),
            password: String::new(),
        }
    }
}
