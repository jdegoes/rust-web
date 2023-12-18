use crate::todoai::models::todo::{CreateTodo, Todo, TodoId, UpdateTodo};
use crate::todoai::services::todoai::TodoAI;
use crate::todoai::services::todorepo::TodoRepo;

#[async_trait::async_trait]
pub trait TodoService: Send + Sync + Clone {
    // async fn create_many(&self, prompt: String) -> Vec<Todo>;
    async fn create(&self, description: String) -> Todo;
    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;
    async fn delete_by_id(&self, id: TodoId) -> bool;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

#[derive(Clone)]
pub struct LiveTodoService<S1: TodoRepo, S2: TodoAI> {
    todo_repo: S1,
    todo_ai: S2,
}
impl<S1: TodoRepo, S2: TodoAI> LiveTodoService<S1, S2> {
    pub fn new(todo_repo: S1, todo_ai: S2) -> Self {
        Self { todo_repo, todo_ai }
    }
}

#[async_trait::async_trait]
impl<S1: TodoRepo, S2: TodoAI> TodoService for LiveTodoService<S1, S2> {
    async fn create(&self, description: String) -> Todo {
        let title = self.todo_ai.infer_title(description.clone()).await.unwrap();
        let deadline = self.todo_ai.infer_deadline(description.clone()).await;
        let priority = self
            .todo_ai
            .infer_priority(description.clone())
            .await
            .unwrap();
        let tags = self.todo_ai.infer_tags(description.clone()).await.unwrap();

        let create_todo = CreateTodo {
            title,
            description,
            deadline,
            tags,
            priority,
        };

        self.todo_repo.create(create_todo).await
    }

    async fn get_by_id(&self, id: TodoId) -> Option<Todo> {
        self.todo_repo.get_by_id(id).await
    }

    async fn delete_by_id(&self, id: TodoId) -> bool {
        self.todo_repo.delete_by_id(id).await
    }

    async fn get_all(&self) -> Vec<Todo> {
        self.todo_repo.get_all().await
    }

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo> {
        self.todo_repo.update(id, update_todo).await
    }
}
