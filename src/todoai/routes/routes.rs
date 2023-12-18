use crate::todoai::controllers::handler::Handler;
use crate::todoai::controllers::todoservice::TodoService;
use axum::routing::*;
use axum::Router;

pub fn make_routes<SERVICE: TodoService + 'static>(todo_service: SERVICE) -> Router {
    Router::new()
        .route("/todos/", post(Handler::<SERVICE>::create_todo))
        .route("/todos/:id", get(Handler::<SERVICE>::get_by_id))
        .route("/todos/:id", delete(Handler::<SERVICE>::delete_by_id))
        .route("/todos/", get(Handler::<SERVICE>::get_all))
        .route("/todos/:id", put(Handler::<SERVICE>::update))
        .with_state(todo_service)
}
