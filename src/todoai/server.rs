use crate::todoai::controllers::todoservice::LiveTodoService;
use crate::todoai::routes::routes::make_routes;
use crate::todoai::services::todoai::OpenAITodoAI;
use crate::todoai::services::todorepo::PostgresTodoRepo;
use axum::Router;
use sqlx::postgres::PgPoolOptions;

pub async fn start() {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let todo_repo = PostgresTodoRepo::new(pool);

    let client = async_openai::Client::new();

    let todo_ai = OpenAITodoAI::new(client);

    let service = LiveTodoService::new(todo_repo, todo_ai);

    // build our application with a route
    let app: Router = make_routes(service);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
