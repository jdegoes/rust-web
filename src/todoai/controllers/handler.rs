use crate::todoai::controllers::todoservice::TodoService;
use crate::todoai::models::todo::{Todo, TodoId, UpdateTodo};
use axum::extract::{Path, State};
use axum::Json;
use std::marker::PhantomData;

pub struct Handler<S: TodoService>(PhantomData<S>);

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetTodosRequest;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct CreateTodoRequest {
    description: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct DeleteByIdRequest {
    id: TodoId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct UpdateTodoRequest {
    update_todo: UpdateTodo,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetByIdRequest {
    id: TodoId,
}

impl<S: TodoService> Handler<S> {
    pub async fn create_todo(
        State(todo_service): State<S>,
        Json(CreateTodoRequest { description }): Json<CreateTodoRequest>,
    ) -> Json<Todo> {
        let todo = todo_service.create(description).await;

        Json(todo)
    }

    pub async fn get_by_id(
        State(todo_service): State<S>,
        Path(GetByIdRequest { id }): Path<GetByIdRequest>,
    ) -> Json<Option<Todo>> {
        let todo = todo_service.get_by_id(id).await;

        Json(todo)
    }

    pub async fn delete_by_id(
        State(todo_service): State<S>,
        Json(DeleteByIdRequest { id }): Json<DeleteByIdRequest>,
    ) -> Json<bool> {
        let result = todo_service.delete_by_id(id).await;

        Json(result)
    }

    pub async fn get_all(State(todo_service): State<S>) -> Json<Vec<Todo>> {
        let todos = todo_service.get_all().await;

        Json(todos)
    }

    pub async fn update(
        State(todo_service): State<S>,
        Path(GetByIdRequest { id }): Path<GetByIdRequest>,
        Json(UpdateTodoRequest { update_todo }): Json<UpdateTodoRequest>,
    ) -> Json<Option<Todo>> {
        let todo = todo_service.update(id, update_todo).await;

        Json(todo)
    }
}
