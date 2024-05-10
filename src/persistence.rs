#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

//!
//! PERSISTENCE
//! -----------
//!
//! Every web application needs to store data. There are, of course, many Rust
//! crates for interacting with NoSQL databases and AWS services like DynamoDB.
//! There are even some ORM-like solutions for Rust that aim to emulate the
//! ORM solutions from the Java world. However, most web applications will rely
//! on relational databases for persistence because of their ubiquity,
//! flexibility, performance, and ACID guarantees.
//!
//! Rust has many solutions for interacting with relational databases. One of
//! the most common that does not try to hide SQL from the user, and which is
//! fully compatible with Tokio, is the `sqlx` crate.
//!
//! In this section, you will learn the basics of using the `sqlx` crate to
//! interact with a PostgreSQL database.
//!
//! To get started:
//!
//! 1. Run `cargo install sqlx-cli` to install the SQLx CLI.
//!
//! 2. Set the environment variable
//! `DATABASE_URL=postgres://<user>:<password>@<address>:<port>/<database>`.
//! For example, `DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres`.
//!
//! 3. Run `sqlx database create` to create the database.
//!
//! 4. Run `sqlx migrate run` to run the migrations in the `migrations` folder.
//!

use sqlx::{postgres::PgPoolOptions, types::time::PrimitiveDateTime, Pool, Postgres};

use axum::{
    async_trait, body::Body, http::{Method, Request}, response::Html, routing::*, Json, Router
};
use axum::extract::State;
use axum::extract::Path;

///
/// EXERCISE 1
///
/// Experiment with the `sqlx::query!` macro. If you have configured your
/// DATABASE_URL correctly (with a running Postgres), then you should be able
/// to get live feedback from the macro.
///
/// At the same time, try the `sqlx::query::<Postgres>` function, which is NOT a macro.
/// What can you say about the difference between the two?
///
/// Note that calling either `query` does not actually execute the query. For that, you
/// need to supply a database pool, which you can do so with the `fetch` family of
/// methods.
///
async fn query_playground() {
    let _ = sqlx::query!("SELECT 1 + 1 AS sum");

    let _ = sqlx::query::<Postgres>("SELECT 1 + 1 AS sum");
}

///
/// EXERCISE 2
///
/// Use the `sqlx::query!` macro to select the result of `1 + 1` from the database,
/// being sure to name the column `sum` using SQL's `AS` keyword.
///
/// Then modify the test to reference a row, which you can obtain by using the
/// `fetch_one` method on the query result, and awaiting and unwrapping it.
///
#[tokio::test]
async fn select_one_plus_one() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _sum: i32 = sqlx::query!("SELECT 1 + 1 AS sum")
        .fetch_one(&_pool).await.unwrap().sum.unwrap();

    assert_eq!(_sum, 2);
}

///
/// EXERCISE 3
///
/// In this example, we are going to show the strength of sqlx by
/// doing a select star query.
///
/// Use the `sqlx::query!` macro to select all columns from the `todos` table.
/// Use a `fetch_all`, and iterate over them, printing out each row.
///
/// What do you notice about the type of the row?
///
#[tokio::test]
async fn select_star() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let todos = sqlx::query!("SELECT * from todos")
        .fetch_all(&_pool).await.unwrap();

    for todo in todos {
        println!("{:?}", todo);
    }

    assert!(true);
}

///
/// EXERCISE 4
///
/// The `query!` macro supports parameterized queries, which you can create using the
/// placeholder syntax '$1', '$2', etc. You then supply these parameters after the
/// main query.
///
/// Use the `query!` macro to insert a row into the `todo` table, keeping
/// in mind every todo has a title, description, and a boolean indicating
/// whether it is done.
///
/// Using the `RETURNING` keyword, return the id of the inserted row,
/// and assert it is greater than zero.
///
#[tokio::test]
async fn insert_todo() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _title = "Learn SQLx";
    let _description = "I should really learn SQLx for my Axum web app";
    let _done = false;

    let _id = sqlx::query!(
        "INSERT INTO todos (title, description, done) VALUES ($1, $2, $3) RETURNING id",
        _title,
        _description,
        _done
    ).fetch_one(&_pool).await.unwrap().id;

    assert!(_id > 0);
}

///
/// EXERCISE 5
///
/// Use the `query!` macro to update a row in the `todo` table.
///
/// You may want to use `execute` to execute the query, rather than one
/// of the fetch methods.
///
#[tokio::test]
async fn update_todo_test() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _id = 1;
    let _done = true;

    sqlx::query!("UPDATE todos SET done = $1 WHERE id = $2", _done, _id)
        .execute(&_pool).await.unwrap();

    assert!(true);
}

///
/// EXERCISE 6
///
/// Use the `query!` macro to delete a row in the `todo` table.
///
/// You may want to use `execute` to execute the query, rather than one
/// of the fetch methods.
///
#[tokio::test]
async fn delete_todo_test() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _id = 1;

    sqlx::query!("DELETE FROM todos WHERE id = $1", _id)
        .execute(&_pool).await.unwrap();

    assert!(true);
}

///
/// EXERCISE 7
///
/// You do not have to rely on SQLx generating anonymous structs for you.
/// With the `sqlx::query_as!` macro, you can specify the type of the row
/// yourself.
///
/// In this exercise, introduce a struct called `Todo` that models the `todos`
/// table, and use the `sqlx::query_as!` macro to select all columns from the
/// `todos` table.
///
#[tokio::test]
async fn select_star_as() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    todo!("Insert query here");

    sqlx::query_as!(TodoPersistence, "SELECT * from todos")
        .fetch_all(&_pool).await.unwrap();

    assert!(true);
}

#[derive(Debug, Clone, PartialEq)]
struct TodoPersistence {
    id: i64,
    title: String,
    description: String,
    done: bool,
    created_at: PrimitiveDateTime,
}

#[async_trait]
trait TodoRepo: Send + Sync + Clone + 'static {
    async fn get_all(&self) -> Vec<Todo>;

    async fn create(&self, title: String, description: String) -> i64;

    async fn get(&self, id: i64) -> Option<Todo>;

    async fn update(&self, id: i64, title: Option<String>, description: Option<String>, done: Option<bool>) -> ();

    async fn delete(&self, id: i64) -> ();
}

#[derive(Debug, Clone)]
struct TodoRepoPostgres {
    pool: Pool<Postgres>,
}

impl TodoRepoPostgres {
    async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(16)
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        Self { pool }
    }
}

#[async_trait]
impl TodoRepo for TodoRepoPostgres {
    async fn get_all(&self) -> Vec<Todo> {
        let todos = sqlx::query!("SELECT * FROM todos")
            .fetch_all(&self.pool).await.unwrap();

        todos.into_iter().map(|todo| {
            Todo {
                id: todo.id,
                title: todo.title,
                description: todo.description,
                done: todo.done,
                created_at: todo.created_at.to_string(),
            }
        }).collect()
    }

    async fn create(&self, title: String, description: String) -> i64 {
        sqlx::query!(
            "INSERT INTO todos (title, description, done) VALUES ($1, $2, false) RETURNING id",
            title,
            description
        ).fetch_one(&self.pool).await.unwrap().id
    }

    async fn get(&self, id: i64) -> Option<Todo> {
        let todo = sqlx::query!("SELECT * FROM todos WHERE id = $1", id)
            .fetch_optional(&self.pool).await.unwrap();

        todo.map(|todo| {
            Todo {
                id: todo.id,
                title: todo.title,
                description: todo.description,
                done: todo.done,
                created_at: todo.created_at.to_string(),
            }
        })
    }

    async fn update(&self, id: i64, title: Option<String>, description: Option<String>, done: Option<bool>) -> () {
        sqlx::query!(
            "UPDATE todos SET title = COALESCE($1, title), description = COALESCE($2, description), done = COALESCE($3, done) where id = $4",
            title,
            description,
            done,
            id,
        ).execute(&self.pool).await.unwrap();
    }

    async fn delete(&self, id: i64) -> () {
        sqlx::query!("DELETE FROM todos WHERE id = $1", id).execute(&self.pool).await.unwrap();
    }
}

fn create_todo_app<R: TodoRepo>(todo_repo: R) -> Router<()> {
    let app = Router::<R>::new()
        .route("/todos", get(get_all_todos::<R>))
        .route("/todos", post(create_todo::<R>))
        .route("/todos/:id", get(get_todo::<R>))
        .route("/todos/:id", put(update_todo::<R>))
        .route("/todos/:id", delete(delete_todo::<R>))
        .with_state(todo_repo);

    app
}

///
/// GRADUATION PROJECT
///
/// In this project, you will build a simple CRUD API for a todo list,
/// which uses sqlx for persistence.
///
pub async fn run_todo_app() {
    let app = create_todo_app(TodoRepoPostgres::new().await);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn get_all_todos<R: TodoRepo>(State(state): State<R>) -> Json<Vec<Todo>> {
    let todos = state.get_all().await;

    Json(todos)
}

async fn create_todo<R: TodoRepo>(State(state): State<R>, Json(create): Json<CreateTodo>) -> Json<CreatedTodo> {
    let id = state.create(create.title.clone(), create.description.clone()).await;

    Json(CreatedTodo { id })
}

async fn get_todo<R: TodoRepo>(State(state): State<R>, Path(id): Path<i64>) -> Json<Option<Todo>> {
    let todo = state.get(id).await;

    Json(todo)
}

async fn update_todo<R: TodoRepo>(State(state): State<R>, Path(id): Path<i64>, Json(update): Json<UpdateTodo>) -> () {
    state.update(id, update.title.clone(), update.description.clone(), update.done).await;
}

async fn delete_todo<R: TodoRepo>(State(state): State<R>, Path(id): Path<i64>) -> () {
    state.delete(id).await;
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
struct CreateTodo {
    title: String,
    description: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Clone)]
struct CreatedTodo {
    id: i64,
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    done: Option<bool>,
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
struct Todo {
    id: i64,
    title: String,
    description: String,
    done: bool,
    created_at: String,
}
