#![allow(dead_code)]

use axum::{response::Html, routing::*, Router};
use sqlx::postgres::PgPoolOptions;

///
/// This example shows you how to use the `testcontainers` crate to run a
/// Postgres database in a Docker container, and how to make basic queries
/// using the `sqlx` crate.
///
/// Be sure to install Docker and associated tooling (daemon, etc.) before
/// running this example.
///
pub async fn example_postgres() -> Result<(), sqlx::Error> {
    use testcontainers::clients;
    use testcontainers_modules::postgres;

    let docker = clients::Cli::default();
    let postgres_instance = docker.run(postgres::Postgres::default());

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        postgres_instance.get_host_port_ipv4(5432)
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    let _rows = sqlx::query("SELECT 1 + 1 AS result").fetch_all(&pool).await;

    Ok(())
}

pub async fn example_axum() {
    // build our application with a route
    let app = Router::new().route("/", get(|| async move { Html("Hello, World!") }));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
