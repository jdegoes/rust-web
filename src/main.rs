mod architecture;
mod basics;
mod client;
mod context;
mod handlers;
mod middleware;
mod persistence;
mod playground;
mod welcome;

#[tokio::main]
async fn main() {
    // playground::example_postgres().await.unwrap();
    // basics::hello_world().await;
    // handlers::run_users_server().await;
    // middleware::prometheus_metrics_middleware().await;
    context::start_router_mutable_context().await;

    // println!("Hello, world!");
}
