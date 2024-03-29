use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub task: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewTask {
    pub task: String,
}
