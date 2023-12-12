mod basics;
mod context;
mod handlers;
mod middleware;
mod playground;
mod welcome;

#[tokio::main]
async fn main() {
    // playground::example_postgres().await.unwrap();

    // basics::hello_world().await;

    handlers::run_users_server().await;

    println!("Hello, world!");
}
