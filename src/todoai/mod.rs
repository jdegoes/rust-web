mod controllers;
mod models;
mod routes;
mod server;
mod services;

pub async fn start() {
    server::start().await;
}
