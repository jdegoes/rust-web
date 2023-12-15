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

use async_openai::{config::OpenAIConfig, types::CreateCompletionRequestArgs};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

//
// EXERCISE 1
// ----------
//
// You are creating a todo app. Furthermore, you want to utilize best
// practice architecture and design patterns that may cost more in
// the short term, but will increase the quality of your solution and
// lower costs of maintenance.
//
// Naturally, you will be using the Rust programming language for this
// project. You have decided to use Axum as your web framework, and
// Postgres as the database, with SQLx as the database library.
//
// Your first step is to design the data structures that will be used
// for modeling different entities in the domain of a todo application.
//
// Use best-practice data modeling techniques to design these data
// structures, deriving implementations for traits like `Clone`,
// `Debug`, `PartialEq`, `Serialize`, and `Deserialize`.
//
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTodo {
    title: String,
    description: String,
    deadline: Option<NaiveDateTime>,
    tags: String,
    priority: Priority,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTodo {
    title: String,
    description: String,
    status: Status,
    deadline: Option<NaiveDateTime>,
    tags: String,
    priority: Priority,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    id: TodoId,
    title: String,
    description: String,
    status: Status,
    created_at: NaiveDateTime,
    deadline: Option<NaiveDateTime>,
    tags: String,
    priority: Priority,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Cancelled,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TodoId(i64);
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
impl Into<i16> for Priority {
    fn into(self) -> i16 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }
}
impl From<i16> for Status {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::Todo,
            1 => Self::InProgress,
            2 => Self::Done,
            3 => Self::Cancelled,
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
            Self::Cancelled => 3,
        }
    }
}

//
// EXERCISE 2
// ----------
//
// Now that you have designed data structures for the todo app, it's
// natural to think about how you might persist them to a database.
//
// Construct a relational data model for your data structures that
// follows best practices for relational data modeling, including
// normalization and foreign keys.
//
// Then write SQLx migration scripts to create the relational data
// model in Postgres. Finally, use the sqlx CLI to run the migration
// on your Postgres database so you are ready to proceed to the next
// step.
//

//
// EXERCISE 3
// ----------
//
// When working on application architecture, your top priorities are
// as follows:
//
// 1. Modularity. You want to be able to break down the application
//   into distinct pieces, each of is devoted to specific and highly
//   scoped functionality. No part of the application should need to
//   know about the internals of any other part of the application.
//   Each part should be independently replaceable.
//
// 2. Testability. You want to be able to test all of your application
//   logic in isolation, without having to cover everything with
//   integration or system tests. This means that your application logic
//   must be independent of real web servers, real database, and real
//   APIs, which implies that you have facades that abstract over
//   the core pieces of functionality required by your logic.
//
// In the exercises in this workshop so far, we have been directly
// talking to databases inside handlers. Moreover, our handlers have
// been thoroughly entangled with the data types of Axum. This close
// coupling to database and web server does not meet the modularity
// and testability requirements of a well-designed application.
//
// Do a thought experiment for the todo app you are developing:
//
// 1. What, if anything, does your application logic require from Axum?
//
// 2. What, if anything, does your application logic require from SQLx?
//
// Now try to scope these requirements to the smallest possible surface
// area. For example, obviously a todo app requires persistence, but it
// does not require the full power of SQL: in fact, only a few different
// SQL queries will be necessary to implement the entire application.
//
// In Rust, traits are essential tools of abstraction. Beyond traits, you
// have your choice of polymorphism or dynamic dispatch (Box<dyn Trait>).
//
// Design a set of traits that abstract over the functionality required
// by your application logic. Can you think of any patterns from other
// programming languages that might be useful here?
//
#[async_trait::async_trait]
trait TodoRepo {
    async fn create(&self, create_todo: CreateTodo) -> Todo;

    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;

    async fn delete_by_id(&self, id: TodoId) -> bool;

    async fn get_all(&self) -> Vec<Todo>;

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

#[async_trait::async_trait]
trait TodoService {
    // async fn create_many(&self, text: String) -> Vec<Todo>;

    async fn create(&self, description: String) -> Todo;

    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;

    async fn delete_by_id(&self, id: TodoId) -> bool;

    async fn get_all(&self) -> Vec<Todo>;

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

#[async_trait::async_trait]
trait TodoAI {
    async fn infer_title(&self, description: &String) -> Option<String>;

    async fn infer_deadline(&self, description: &String) -> Option<NaiveDateTime>;

    async fn infer_priority(&self, description: &String) -> Option<Priority>;

    async fn infer_tags(&self, description: &String) -> Option<String>;
}

//
// EXERCISE 4
// ----------
//
// Now that you have designed a set of traits that abstract over the
// functionality required by your application logic, you can implement
// these traits for data types that are bound to Axum and SQLx.
//
// The implementations will be "live" implementations that directly
// talk to Axum and SQLx, but they will be hidden behind the traits
// you designed in the previous exercise.
//
pub struct PostgresTodoRepo {
    pool: Pool<Postgres>,
}

impl PostgresTodoRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn create(&self, create_todo: CreateTodo) -> Todo {
        let priority: i16 = create_todo.priority.into();
        let status: i16 = Status::Todo.into();

        let result = sqlx::query!(
            "INSERT INTO todos (title, description, status, deadline, tags, priority) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, title, description, status, created_at, deadline, tags, priority",
            create_todo.title,
            create_todo.description,
            status,
            create_todo.deadline,
            create_todo.tags,
            priority,
        ).fetch_one(&self.pool).await.unwrap();

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
        let optional = sqlx::query!(
            "SELECT id, title, description, status, created_at, deadline, tags, priority FROM todos WHERE id = $1",
            id.0,
        ).fetch_optional(&self.pool).await.unwrap();

        match optional {
            Some(result) => Some(Todo {
                id: TodoId(result.id),
                title: result.title,
                description: result.description.unwrap_or("".to_string()),
                status: Status::from(result.status),
                created_at: result.created_at,
                deadline: result.deadline,
                tags: result.tags,
                priority: Priority::from(result.priority),
            }),
            None => None,
        }
    }

    async fn delete_by_id(&self, id: TodoId) -> bool {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id.0,)
            .execute(&self.pool)
            .await
            .unwrap();

        result.rows_affected() > 0
    }

    async fn get_all(&self) -> Vec<Todo> {
        let results = sqlx::query!(
            "SELECT id, title, description, status, created_at, deadline, tags, priority FROM todos",
        ).fetch_all(&self.pool).await.unwrap();

        results
            .into_iter()
            .map(|result| Todo {
                id: TodoId(result.id),
                title: result.title,
                description: result.description.unwrap_or("".to_string()),
                status: Status::from(result.status),
                created_at: result.created_at,
                deadline: result.deadline,
                tags: result.tags,
                priority: Priority::from(result.priority),
            })
            .collect()
    }

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo> {
        let priority: i16 = update_todo.priority.into();
        let status: i16 = update_todo.status.into();

        let result = sqlx::query!(
            "UPDATE todos SET title = $1, description = $2, status = $3, deadline = $4, tags = $5, priority = $6 WHERE id = $7 RETURNING id, title, description, status, created_at, deadline, tags, priority",
            update_todo.title,
            update_todo.description,
            status,
            update_todo.deadline,
            update_todo.tags,
            priority,
            id.0,
        ).fetch_optional(&self.pool).await.unwrap();

        match result {
            Some(result) => Some(Todo {
                id: TodoId(result.id),
                title: result.title,
                description: result.description.unwrap_or("".to_string()),
                status: Status::from(result.status),
                created_at: result.created_at,
                deadline: result.deadline,
                tags: result.tags,
                priority: Priority::from(result.priority),
            }),
            None => None,
        }
    }
}

pub struct OpenAITodoAI {
    client: async_openai::Client<OpenAIConfig>,
}
impl OpenAITodoAI {
    pub fn new(client: async_openai::Client<OpenAIConfig>) -> Self {
        Self { client }
    }
}
/*
// Create request using builder pattern
 // Every request struct has companion builder struct with same name + Args suffix
 let request = CreateCompletionRequestArgs::default()
     .model("text-davinci-003")
     .prompt("Tell me the recipe of alfredo pasta")
     .max_tokens(40_u16)
     .build()
     .unwrap();

 // Call API
 let response = client
     .completions()      // Get the API "group" (completions, images, etc.) from the client
     .create(request)    // Make the API call in that "group"
     .await
     .unwrap();
 */
#[async_trait::async_trait]
impl TodoAI for OpenAITodoAI {
    async fn infer_title(&self, description: &String) -> Option<String> {
        let prompt = 
            format!("I need you to infer a short title (200 characters max) for the following todo description. Please 
                reply ONLY with the title and do not explain yourself in any way. Description: {}", description);

        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(prompt)
            .max_tokens(40_u16)
            .build()
            .unwrap();
   
        let response = self.client
            .completions()
            .create(request)
            .await
            .unwrap();

        response.choices.first().map(|first| { return first.text.clone() })
    }

    async fn infer_deadline(&self, description: &String) -> Option<NaiveDateTime> {
        None
    }

    async fn infer_priority(&self, description: &String) -> Option<Priority> {
        let prompt = 
            format!("I am going to give you a description for a TODO. I want you to reply with an estimated priority for the TODO,
            ranging from 0 (low priority) to 1 (medium priority) to 2 (high priority). Only reply with the number 1, 2, or 3,
            depending on how you estimate the priority of the TODO. Description: {}", description);

        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(prompt)
            .max_tokens(40_u16)
            .build()
            .unwrap();
   
        let response = self.client
            .completions()
            .create(request)
            .await
            .unwrap();

        response.choices.first().map(|first| { return first.text.clone().parse::<i16>().unwrap().into() })
    }

    async fn infer_tags(&self, description: &String) -> Option<String> {
        let prompt = 
            format!("I need you to infer a comma-separated list of tags that describe the high-level concepts 
             related to the following TODO description. Only reply with the comma-separated list of tags. Each tag
             should be a single word (no symbols). Description: {}", description);

        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(prompt)
            .max_tokens(40_u16)
            .build()
            .unwrap();
   
        let response = self.client
            .completions()
            .create(request)
            .await
            .unwrap();

        response.choices.first().map(|first| { return first.text.clone() })
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
