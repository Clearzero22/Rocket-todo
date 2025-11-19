//! Subtask Service Implementation

use crate::services::{error::{ServiceError, ServiceResult}, DbConnection};
use crate::models::{CreateSubtaskRequest, UpdateSubtaskRequest, Subtask, SubtaskResponse};

/// Subtask service handles subtask-related business logic
#[derive(Debug)]
pub struct SubtaskService;

impl SubtaskService {
    /// Create a new SubtaskService instance
    pub fn new() -> Self {
        Self
    }

    /// Create a new subtask
    pub async fn create(
        &self,
        mut db: DbConnection,
        request: CreateSubtaskRequest,
    ) -> ServiceResult<SubtaskResponse> {
        // Validate request
        if request.title.trim().is_empty() {
            return Err(ServiceError::validation("Subtask title cannot be empty"));
        }

        // Check if parent todo exists
        let _parent = sqlx::query!("SELECT id FROM todos WHERE id = ?", request.parent_todo_id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Todo with id {}", request.parent_todo_id)))?;

        // Insert subtask
        let result = sqlx::query!(
            "INSERT INTO subtasks (parent_todo_id, title, description, status, due_date) VALUES (?, ?, ?, ?, ?)",
            request.parent_todo_id,
            request.title,
            request.description,
            request.status.as_str(),
            request.due_date
        )
        .execute(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        let id = result.last_insert_rowid();

        // Get the created record
        let subtask = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks WHERE id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(SubtaskResponse::from(subtask))
    }

    /// Get a subtask by ID
    pub async fn get_by_id(&self, mut db: DbConnection, id: i64) -> ServiceResult<SubtaskResponse> {
        let subtask = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks WHERE id = ?",
            id
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("Subtask with id {}", id)))?;

        Ok(SubtaskResponse::from(subtask))
    }

    /// Get all subtasks
    pub async fn list(&self, mut db: DbConnection) -> ServiceResult<Vec<SubtaskResponse>> {
        let subtasks = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks ORDER BY created_at DESC"
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(subtasks.into_iter().map(SubtaskResponse::from).collect())
    }

    /// Get subtasks by parent todo ID
    pub async fn get_by_parent_todo_id(&self, mut db: DbConnection, parent_todo_id: i64) -> ServiceResult<Vec<SubtaskResponse>> {
        // Check if parent todo exists
        let _parent = sqlx::query!("SELECT id FROM todos WHERE id = ?", parent_todo_id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Todo with id {}", parent_todo_id)))?;

        let subtasks = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks WHERE parent_todo_id = ? ORDER BY created_at ASC",
            parent_todo_id
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(subtasks.into_iter().map(SubtaskResponse::from).collect())
    }

    /// Update a subtask
    pub async fn update(
        &self,
        mut db: DbConnection,
        id: i64,
        request: UpdateSubtaskRequest,
    ) -> ServiceResult<SubtaskResponse> {
        // Check if subtask exists
        let _existing = sqlx::query!("SELECT id FROM subtasks WHERE id = ?", id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Subtask with id {}", id)))?;

        // Build update query dynamically
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(ServiceError::validation("Subtask title cannot be empty"));
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

        if let Some(ref due_date) = request.due_date {
            updates.push("due_date = ?");
            params.push(due_date.clone());
        }

        if !updates.is_empty() {
            updates.push("updated_at = CURRENT_TIMESTAMP");
            let query = format!("UPDATE subtasks SET {} WHERE id = ?", updates.join(", "));

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

    /// Delete a subtask
    pub async fn delete(&self, mut db: DbConnection, id: i64) -> ServiceResult<()> {
        let result = sqlx::query!("DELETE FROM subtasks WHERE id = ?", id)
            .execute(&mut **db)
            .await
            .map_err(ServiceError::Database)?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::not_found(&format!("Subtask with id {}", id)));
        }

        Ok(())
    }

    /// Get overdue subtasks
    pub async fn get_overdue(&self, mut db: DbConnection) -> ServiceResult<Vec<SubtaskResponse>> {
        let subtasks = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks
             WHERE due_date < datetime('now') AND status != 'completed'
             ORDER BY due_date ASC"
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(subtasks.into_iter().map(SubtaskResponse::from).collect())
    }

    /// Get overdue subtasks for a specific todo
    pub async fn get_overdue_for_todo(&self, mut db: DbConnection, parent_todo_id: i64) -> ServiceResult<Vec<SubtaskResponse>> {
        // Check if parent todo exists
        let _parent = sqlx::query!("SELECT id FROM todos WHERE id = ?", parent_todo_id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Todo with id {}", parent_todo_id)))?;

        let subtasks = sqlx::query_as!(
            Subtask,
            "SELECT id, parent_todo_id, title, description, status, due_date, created_at, updated_at
             FROM subtasks
             WHERE parent_todo_id = ? AND due_date < datetime('now') AND status != 'completed'
             ORDER BY due_date ASC",
            parent_todo_id
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(subtasks.into_iter().map(SubtaskResponse::from).collect())
    }
}

impl Default for SubtaskService {
    fn default() -> Self {
        Self::new()
    }
}