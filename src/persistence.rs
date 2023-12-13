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

use sqlx::{postgres::PgPoolOptions, Postgres};

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

    let _sum: i32 = todo!("Insert row here");

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

    todo!("Insert query here");

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

    assert!(true);
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
async fn update_todo() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _id = 1;
    let _done = true;

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
async fn delete_todo() {
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _id = 1;

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

    assert!(true);
}

///
/// GRADUATION PROJECT
///
/// In this project, you will build a simple CRUD API for a todo list,
/// which uses sqlx for persistence.
///
pub async fn run_todo_app() {
    todo!("Implement todo app");
}
