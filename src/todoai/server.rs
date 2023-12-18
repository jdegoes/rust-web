use crate::todoai::routes::routes::make_routes;
use axum::Router;

pub async fn start() {
    // build our application with a route
    let app: Router = make_routes();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
