use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}
