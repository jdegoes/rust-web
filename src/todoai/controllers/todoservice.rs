use crate::todoai::models::{CreateTodo, Priority, Status, Todo, TodoId, UpdateTodo};
use crate::todoai::services::TodoAI;
use async_openai::{config::OpenAIConfig, types::CreateCompletionRequestArgs};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[async_trait::async_trait]
trait TodoService {
    async fn create_many(&self, prompt: String) -> Vec<Todo>;
    async fn create(&self, description: String) -> Todo;
    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;
    async fn delete_by_id(&self, id: TodoId) -> bool;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}
