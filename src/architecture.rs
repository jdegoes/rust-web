//!
//! ARCHITECTURE
//! ------------
//!
//! This module allows an exploration of how you might structure a larger
//! Axum application. It is the final graduation project for the course.
//!
//! In particular, the emphasis is on modularity and testability: being
//! able to break down the functionality of the whole app into distinct
//! pieces, each of which may be tested independently, and without having
//! to cover everything with integration or system tests.
//!
//! To achieve a modular and testable design, we can take advantage of
//! both features of Axum and features of the Rust programming language
//! itself, as well as learning lessons from modularity and testability
//! in other languages that have long been used for building web apps.
//!
//!

use async_openai::{config::OpenAIConfig, types::CreateCompletionRequestArgs};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CreateTodo {
    title: String,
    description: String,
    priority: Priority,
    deadline: Option<NaiveDateTime>,
    tags: String,
    // subtasks: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UpdateTodo {
    title: String,
    description: String,
    status: Status,
    priority: Priority,
    deadline: Option<NaiveDateTime>,
    tags: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Todo {
    id: TodoId,
    title: String,
    description: String,
    status: Status,
    priority: Priority,
    created_at: NaiveDateTime,
    deadline: Option<NaiveDateTime>,
    tags: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TodoId(i64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done,
    Aborted,
}

impl From<i16> for Status {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::Todo,
            1 => Self::InProgress,
            2 => Self::Done,
            -1 => Self::Aborted,
            _ => panic!("Invalid status value"),
        }
    }
}

impl Into<i16> for Status {
    fn into(self) -> i16 {
        match self {
            Self::Todo => 0,
            Self::InProgress => 1,
            Self::Done => 2,
            Self::Aborted => -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl From<i16> for Priority {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            _ => panic!("Invalid priority value"),
        }
    }
}

impl From<String> for Priority {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Low" => Self::Low,
            "Medium" => Self::Medium,
            "High" => Self::High,
            t => panic!("Invalid priority value: {}", t),
        }
    }
}

impl Into<i16> for Priority {
    fn into(self) -> i16 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }
}

#[async_trait::async_trait]
trait TodoRepo {
    async fn create(&self, create_todo: CreateTodo) -> Todo;
    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;
    async fn delete_by_id(&self, id: TodoId) -> bool;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

#[async_trait::async_trait]
trait TodoAI {
    async fn infer_title(&self, text: String) -> Option<String>;
    async fn infer_deadline(&self, todo: &Todo) -> Option<NaiveDateTime>;
    async fn infer_priority(&self, todo: &Todo) -> Option<Priority>;
    async fn split_into_todos(&self, prompt: String) -> Vec<Todo>;
    async fn classify(&self, todo: &Todo) -> Vec<String>;
}

#[async_trait::async_trait]
trait TodoService {
    async fn create_many(&self, prompt: String) -> Vec<Todo>;
    async fn create(&self, description: String) -> Todo;
    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;
    async fn delete_by_id(&self, id: TodoId) -> bool;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

struct TodoRepoSqlImpl {
    pool: Pool<Postgres>,
}

impl TodoRepoSqlImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoSqlImpl {
    async fn create(&self, create_todo: CreateTodo) -> Todo {
        let result = sqlx::query!(
            r#"
            INSERT INTO todos (title, description, priority, deadline, tags)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, created_at, status, priority, deadline, tags
            "#,
            create_todo.title,
            create_todo.description,
            create_todo.priority as i16,
            create_todo.deadline,
            create_todo.tags,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        }
    }

    async fn get_by_id(&self, id: TodoId) -> Option<Todo> {
        let result = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, status, priority, deadline, tags
            FROM todos
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .ok()?;

        Some(Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        })
    }

    async fn delete_by_id(&self, id: TodoId) -> bool {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .unwrap();

        result.rows_affected() > 0
    }

    async fn get_all(&self) -> Vec<Todo> {
        let result = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, status, priority, deadline, tags
            FROM todos
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        result
            .into_iter()
            .map(|row| Todo {
                id: TodoId(row.id),
                title: row.title,
                description: row.description.unwrap_or("".to_string()),
                status: Status::from(row.status),
                created_at: row.created_at,
                deadline: row.deadline,
                tags: row.tags,
                priority: Priority::from(row.priority),
            })
            .collect()
    }

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo> {
        let result = sqlx::query!(
            r#"
            UPDATE todos
            SET title = $2, description = $3, status = $4, priority = $5, deadline = $6, tags = $7
            WHERE id = $1
            RETURNING id, title, description, created_at, status, priority, deadline, tags
            "#,
            id.0,
            update_todo.title,
            update_todo.description,
            update_todo.status as i16,
            update_todo.priority as i16,
            update_todo.deadline,
            update_todo.tags,
        )
        .fetch_one(&self.pool)
        .await
        .ok()?;

        Some(Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        })
    }
}

struct TodoOpenAIImpl {
    // https://github.com/64bit/async-openai/tree/main/examples/assistants
    client: async_openai::Client<OpenAIConfig>,
}

impl TodoOpenAIImpl {
    pub fn new(client: async_openai::Client<OpenAIConfig>) -> Self {
        Self { client }
    }

    async fn send_prompt(
        client: &async_openai::Client<OpenAIConfig>,
        prompt: String,
    ) -> Option<String> {
        let request = CreateCompletionRequestArgs::default()
            .model("gpt-3.5")
            .prompt(prompt)
            .max_tokens(40_u16)
            .build()
            .unwrap();

        let response = client.completions().create(request).await.unwrap();

        response
            .choices
            .first()
            .map(|first| return first.text.clone())
    }
}

#[async_trait::async_trait]
impl TodoAI for TodoOpenAIImpl {
    async fn infer_title(&self, text: String) -> Option<String> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the title of the task.

        Description: "{}"

        Only respond with the title and nothing else.
        "#,
            text
        );

        TodoOpenAIImpl::send_prompt(&self.client, prompt).await
    }

    async fn infer_deadline(&self, todo: &Todo) -> Option<NaiveDateTime> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the deadline of the task.
        Here is today's date: {}
        Here is the description: "{}"
        Respond with a date in the format: YYYY-MM-DD
        "#,
            chrono::Local::now().naive_local().date(),
            todo.description
        );

        TodoOpenAIImpl::send_prompt(&self.client, prompt)
            .await
            .map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d")
                    .unwrap()
                    .date()
                    .and_hms_opt(0, 0, 0)
            })
            .flatten()
    }

    async fn infer_priority(&self, todo: &Todo) -> Option<Priority> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the priority of the task.
        The options are: Low, Medium, High
        Only respond with the priority and nothing else.
        Here is the description: "{}"
        "#,
            todo.description
        );

        TodoOpenAIImpl::send_prompt(&self.client, prompt)
            .await
            .map(|s| Priority::from(s))
    }

    async fn split_into_todos(&self, _prompt: String) -> Vec<Todo> {
        vec![]
    }

    async fn classify(&self, _todo: &Todo) -> Vec<String> {
        vec![]
    }
}

//
// EXERCISE 5
// ----------
//
// Now it is time to write your application logic, in a way that is not
// tied to Axum or SQLx. You will use the traits you designed in the
// previous exercise to write your application logic. Avoid using any
// data types directly from Axum or SQLx.
//
// A key test of your application architecture is whether or not you can
// write unit tests for the logic that do not require any real web servers,
// real databases, or real APIs. So as you develop your application logic,
// be sure to introduce tests, which might necessitate you providing
// alternate implementations of the traits you designed previously.
//

//
// EXERCISE 6
// ----------
//
// Now that you have written and tested your application logic, you can use
// Axum to develop routes, with handlers that call into your application logic.
// Take care to wire up everything correctly for production operation.
// Start your web server and verify its behavior matches your expectations.
//
