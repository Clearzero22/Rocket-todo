//! Todo Service Implementation

use crate::services::{error::{ServiceError, ServiceResult}, DbConnection};
use crate::models::{CreateTodoRequest, UpdateTodoRequest, Priority, Status, Todo, TodoResponse};

/// Todo service handles todo-related business logic
#[derive(Debug)]
pub struct TodoService;

impl TodoService {
    /// Create a new TodoService instance
    pub fn new() -> Self {
        Self
    }

    /// Create a new todo
    pub async fn create(
        &self,
        mut db: DbConnection,
        request: CreateTodoRequest,
    ) -> ServiceResult<TodoResponse> {
        // Validate request
        if request.title.trim().is_empty() {
            return Err(ServiceError::validation("Title cannot be empty"));
        }

        let status = request.status.unwrap_or(Status::Pending);
        let priority = request.priority.unwrap_or(Priority::Medium);

        let result = sqlx::query!(
            "INSERT INTO todos (title, description, status, priority, due_date) VALUES (?, ?, ?, ?, ?)",
            request.title,
            request.description,
            status.as_str(),
            priority.as_str(),
            request.due_date
        )
        .execute(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        let id = result.last_insert_rowid();

        // Get the created record
        let todo = sqlx::query_as!(
            Todo,
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at
             FROM todos WHERE id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(TodoResponse::from(todo))
    }

    /// Get a todo by ID
    pub async fn get_by_id(&self, mut db: DbConnection, id: i64) -> ServiceResult<TodoResponse> {
        let todo = sqlx::query_as!(
            Todo,
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at
             FROM todos WHERE id = ?",
            id
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("Todo with id {}", id)))?;

        Ok(TodoResponse::from(todo))
    }

    /// Get all todos
    pub async fn list(&self, mut db: DbConnection) -> ServiceResult<Vec<TodoResponse>> {
        let todos = sqlx::query_as!(
            Todo,
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at
             FROM todos ORDER BY created_at DESC"
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(todos.into_iter().map(TodoResponse::from).collect())
    }

    /// Update a todo
    pub async fn update(
        &self,
        mut db: DbConnection,
        id: i64,
        request: UpdateTodoRequest,
    ) -> ServiceResult<TodoResponse> {
        // Check if todo exists
        let _existing = sqlx::query!("SELECT id FROM todos WHERE id = ?", id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Todo with id {}", id)))?;

        // Build update query dynamically
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(ServiceError::validation("Title cannot be empty"));
            }
            updates.push("title = ?");
            params.push(title.clone());
        }

        if let Some(ref description) = request.description {
            updates.push("description = ?");
            params.push(description.clone());
        }

        if let Some(ref status) = request.status {
            updates.push("status = ?");
            params.push(status.as_str().to_string());
        }

        if let Some(ref priority) = request.priority {
            updates.push("priority = ?");
            params.push(priority.as_str().to_string());
        }

        if let Some(ref due_date) = request.due_date {
            updates.push("due_date = ?");
            params.push(due_date.clone());
        }

        if !updates.is_empty() {
            updates.push("updated_at = CURRENT_TIMESTAMP");
            let query = format!("UPDATE todos SET {} WHERE id = ?", updates.join(", "));

            let mut query_builder = sqlx::query(&query);
            for param in params {
                query_builder = query_builder.bind(param);
            }
            query_builder = query_builder.bind(id);

            query_builder.execute(&mut **db)
                .await
                .map_err(ServiceError::Database)?;
        }

        self.get_by_id(db, id).await
    }

    /// Delete a todo
    pub async fn delete(&self, mut db: DbConnection, id: i64) -> ServiceResult<()> {
        let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
            .execute(&mut **db)
            .await
            .map_err(ServiceError::Database)?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::not_found(&format!("Todo with id {}", id)));
        }

        Ok(())
    }

    /// Get todos by status
    pub async fn get_by_status(&self, mut db: DbConnection, status: Status) -> ServiceResult<Vec<TodoResponse>> {
        let todos = sqlx::query_as!(
            Todo,
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at
             FROM todos WHERE status = ? ORDER BY created_at DESC",
            status.as_str()
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(todos.into_iter().map(TodoResponse::from).collect())
    }

    /// Get todos by priority
    pub async fn get_by_priority(&self, mut db: DbConnection, priority: Priority) -> ServiceResult<Vec<TodoResponse>> {
        let todos = sqlx::query_as!(
            Todo,
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at
             FROM todos WHERE priority = ? ORDER BY created_at DESC",
            priority.as_str()
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(todos.into_iter().map(TodoResponse::from).collect())
    }
}

impl Default for TodoService {
    fn default() -> Self {
        Self::new()
    }
}