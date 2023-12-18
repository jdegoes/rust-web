use crate::todoai::models::priority::Priority;
use crate::todoai::models::status::Status;
use crate::todoai::models::todo::{CreateTodo, Todo, TodoId, UpdateTodo};
use sqlx::{Pool, Postgres};

#[async_trait::async_trait]
pub trait TodoRepo: Send + Sync + Clone {
    async fn create(&self, create_todo: CreateTodo) -> Todo;
    async fn get_by_id(&self, id: TodoId) -> Option<Todo>;
    async fn delete_by_id(&self, id: TodoId) -> bool;
    async fn get_all(&self) -> Vec<Todo>;
    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo>;
}

#[derive(Clone)]
pub struct PostgresTodoRepo {
    pool: Pool<Postgres>,
}

impl PostgresTodoRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn create(&self, create_todo: CreateTodo) -> Todo {
        let result = sqlx::query!(
            r#"
            INSERT INTO todos (title, description, priority, deadline, tags)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, created_at, status, priority, deadline, tags
            "#,
            create_todo.title,
            create_todo.description,
            create_todo.priority as i16,
            create_todo.deadline,
            create_todo.tags,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        }
    }

    async fn get_by_id(&self, id: TodoId) -> Option<Todo> {
        let result = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, status, priority, deadline, tags
            FROM todos
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .ok()?;

        Some(Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        })
    }

    async fn delete_by_id(&self, id: TodoId) -> bool {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .unwrap();

        result.rows_affected() > 0
    }

    async fn get_all(&self) -> Vec<Todo> {
        let result = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, status, priority, deadline, tags
            FROM todos
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        result
            .into_iter()
            .map(|row| Todo {
                id: TodoId(row.id),
                title: row.title,
                description: row.description.unwrap_or("".to_string()),
                status: Status::from(row.status),
                created_at: row.created_at,
                deadline: row.deadline,
                tags: row.tags,
                priority: Priority::from(row.priority),
            })
            .collect()
    }

    async fn update(&self, id: TodoId, update_todo: UpdateTodo) -> Option<Todo> {
        let result = sqlx::query!(
            r#"
            UPDATE todos
            SET title = $2, description = $3, status = $4, priority = $5, deadline = $6, tags = $7
            WHERE id = $1
            RETURNING id, title, description, created_at, status, priority, deadline, tags
            "#,
            id.0,
            update_todo.title,
            update_todo.description,
            update_todo.status as i16,
            update_todo.priority as i16,
            update_todo.deadline,
            update_todo.tags,
        )
        .fetch_one(&self.pool)
        .await
        .ok()?;

        Some(Todo {
            id: TodoId(result.id),
            title: result.title,
            description: result.description.unwrap_or("".to_string()),
            status: Status::from(result.status),
            created_at: result.created_at,
            deadline: result.deadline,
            tags: result.tags,
            priority: Priority::from(result.priority),
        })
    }
}
