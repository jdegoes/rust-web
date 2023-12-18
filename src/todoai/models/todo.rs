use crate::todoai::models::priority::Priority;
use crate::todoai::models::status::Status;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub deadline: Option<NaiveDateTime>,
    pub tags: String,
    // subtasks: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub deadline: Option<NaiveDateTime>,
    pub tags: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: NaiveDateTime,
    pub deadline: Option<NaiveDateTime>,
    pub tags: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoId(pub i64);
