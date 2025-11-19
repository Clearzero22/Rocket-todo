//! Tag Service Implementation

use crate::services::{error::{ServiceError, ServiceResult}, DbConnection};
use crate::models::{CreateTagRequest, UpdateTagRequest, Tag, TagResponse};

/// Tag service handles tag-related business logic
#[derive(Debug)]
pub struct TagService;

impl TagService {
    /// Create a new TagService instance
    pub fn new() -> Self {
        Self
    }

    /// Create a new tag
    pub async fn create(
        &self,
        mut db: DbConnection,
        request: CreateTagRequest,
    ) -> ServiceResult<TagResponse> {
        // Validate request
        if request.name.trim().is_empty() {
            return Err(ServiceError::validation("Tag name cannot be empty"));
        }

        // Check if tag already exists
        let existing = sqlx::query!("SELECT id FROM tags WHERE name = ?", request.name)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?;

        if existing.is_some() {
            return Err(ServiceError::conflict("Tag already exists"));
        }

        // Insert tag
        let result = sqlx::query!(
            "INSERT INTO tags (name, color, description) VALUES (?, ?, ?)",
            request.name,
            request.color,
            request.description
        )
        .execute(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        let id = result.last_insert_rowid();

        // Get the created record
        let tag = sqlx::query_as!(
            Tag,
            "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(TagResponse::from(tag))
    }

    /// Get a tag by ID
    pub async fn get_by_id(&self, mut db: DbConnection, id: i64) -> ServiceResult<TagResponse> {
        let tag = sqlx::query_as!(
            Tag,
            "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
            id
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("Tag with id {}", id)))?;

        Ok(TagResponse::from(tag))
    }

    /// Get all tags
    pub async fn list(&self, mut db: DbConnection) -> ServiceResult<Vec<TagResponse>> {
        let tags = sqlx::query_as!(
            Tag,
            "SELECT id, name, color, description, created_at, updated_at FROM tags ORDER BY created_at DESC"
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(tags.into_iter().map(TagResponse::from).collect())
    }

    /// Update a tag
    pub async fn update(
        &self,
        mut db: DbConnection,
        id: i64,
        request: UpdateTagRequest,
    ) -> ServiceResult<TagResponse> {
        // Check if tag exists
        let _existing = sqlx::query!("SELECT id FROM tags WHERE id = ?", id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Tag with id {}", id)))?;

        // Build update query dynamically
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(ServiceError::validation("Tag name cannot be empty"));
            }
            updates.push("name = ?");
            params.push(name.clone());
        }

        if let Some(ref color) = request.color {
            updates.push("color = ?");
            params.push(color.clone());
        }

        if let Some(ref description) = request.description {
            updates.push("description = ?");
            params.push(description.clone());
        }

        if !updates.is_empty() {
            updates.push("updated_at = CURRENT_TIMESTAMP");
            let query = format!("UPDATE tags SET {} WHERE id = ?", updates.join(", "));

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

    /// Delete a tag
    pub async fn delete(&self, mut db: DbConnection, id: i64) -> ServiceResult<()> {
        let result = sqlx::query!("DELETE FROM tags WHERE id = ?", id)
            .execute(&mut **db)
            .await
            .map_err(ServiceError::Database)?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::not_found(&format!("Tag with id {}", id)));
        }

        Ok(())
    }

    /// Get todos by tag
    pub async fn get_todos_by_tag(&self, mut db: DbConnection, tag_id: i64) -> ServiceResult<Vec<crate::models::TodoResponse>> {
        // Check if tag exists
        let _existing = sqlx::query!("SELECT id FROM tags WHERE id = ?", tag_id)
            .fetch_optional(&mut **db)
            .await
            .map_err(ServiceError::Database)?
            .ok_or_else(|| ServiceError::not_found(&format!("Tag with id {}", tag_id)))?;

        let todos = sqlx::query_as!(
            crate::models::Todo,
            "SELECT t.id, t.title, t.description, t.status, t.priority, t.due_date, t.created_at, t.updated_at
             FROM todos t
             INNER JOIN todo_tags tt ON t.id = tt.todo_id
             WHERE tt.tag_id = ?
             ORDER BY t.created_at DESC",
            tag_id
        )
        .fetch_all(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(todos.into_iter().map(crate::models::TodoResponse::from).collect())
    }
}

impl Default for TagService {
    fn default() -> Self {
        Self::new()
    }
}