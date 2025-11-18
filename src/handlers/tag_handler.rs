use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use crate::database::{Db, DbResult};
use crate::models::{
    Tag, TagResponse, CreateTagRequest, UpdateTagRequest, TodoTag, AddTagRequest,
    TodoWithTagsResponse,
};
use crate::models::{Todo, Priority, Status as TodoStatus};
use rocket_db_pools::Connection;

// Tag CRUD operations

pub async fn create_tag(
    mut db: Connection<Db>,
    request: Json<CreateTagRequest>,
) -> DbResult<Json<TagResponse>> {
    let color = request.color.as_deref().unwrap_or("#007bff");

    let result = sqlx::query!(
        "INSERT INTO tags (name, color, description) VALUES (?, ?, ?)",
        request.name,
        color,
        request.description
    )
    .execute(&mut **db)
    .await?;

    let id = result.last_insert_rowid();

    // Get the created record
    let created_record = sqlx::query!(
        "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await?;

    let tag = Tag {
        id: Some(created_record.id),
        name: created_record.name,
        color: created_record.color,
        description: created_record.description,
        created_at: created_record.created_at,
        updated_at: created_record.updated_at,
    };

    Ok(Json(TagResponse::from(tag)))
}

pub async fn get_tag(
    mut db: Connection<Db>,
    id: i64,
) -> Result<Json<TagResponse>, NotFound<String>> {
    let result = sqlx::query!(
        "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Tag not found: {}", e)))?;

    let tag = Tag {
        id: Some(result.id),
        name: result.name,
        color: result.color,
        description: result.description,
        created_at: result.created_at,
        updated_at: result.updated_at,
    };

    Ok(Json(TagResponse::from(tag)))
}

pub async fn get_all_tags(mut db: Connection<Db>) -> DbResult<Json<Vec<TagResponse>>> {
    let results = sqlx::query!(
        "SELECT id, name, color, description, created_at, updated_at FROM tags ORDER BY created_at DESC"
    )
    .fetch_all(&mut **db)
    .await?;

    let tags: Vec<TagResponse> = results
        .into_iter()
        .map(|row| {
            let tag = Tag {
                id: Some(row.id),
                name: row.name,
                color: row.color,
                description: row.description,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TagResponse::from(tag)
        })
        .collect();

    Ok(Json(tags))
}

pub async fn update_tag(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateTagRequest>,
) -> Result<Json<TagResponse>, NotFound<String>> {
    // First, get the existing tag
    let existing = sqlx::query!(
        "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Tag not found: {}", e)))?;

    // Build update query dynamically
    let mut update_fields = Vec::new();
    let mut updates = Vec::new();

    if let Some(name) = &request.name {
        update_fields.push("name = ?");
        updates.push(name.clone());
    }
    if let Some(color) = &request.color {
        update_fields.push("color = ?");
        updates.push(color.clone());
    }
    if let Some(description) = &request.description {
        update_fields.push("description = ?");
        updates.push(description.clone());
    }

    if update_fields.is_empty() {
        // No fields to update, return existing tag
        let tag = Tag {
            id: Some(existing.id),
            name: existing.name,
            color: existing.color,
            description: existing.description,
            created_at: existing.created_at,
            updated_at: existing.updated_at,
        };
        return Ok(Json(TagResponse::from(tag)));
    }

    update_fields.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!("UPDATE tags SET {} WHERE id = ?", update_fields.join(", "));

    // Execute the update
    let mut query_builder = sqlx::query(&query);
    for update in updates {
        query_builder = query_builder.bind(update);
    }
    query_builder = query_builder.bind(id);

    let result = query_builder
        .execute(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Failed to update tag: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(NotFound("Tag not found".to_string()));
    }

    // Get the updated record
    let updated_record = sqlx::query!(
        "SELECT id, name, color, description, created_at, updated_at FROM tags WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to fetch updated tag: {}", e)))?;

    let tag = Tag {
        id: Some(updated_record.id),
        name: updated_record.name,
        color: updated_record.color,
        description: updated_record.description,
        created_at: updated_record.created_at,
        updated_at: updated_record.updated_at,
    };

    Ok(Json(TagResponse::from(tag)))
}

pub async fn delete_tag(mut db: Connection<Db>, id: i64) -> Result<Status, NotFound<String>> {
    let result = sqlx::query!("DELETE FROM tags WHERE id = ?", id)
        .execute(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Failed to delete tag: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(NotFound("Tag not found".to_string()));
    }

    Ok(Status::NoContent)
}

// Todo-Tag association operations

pub async fn add_tag_to_todo(
    mut db: Connection<Db>,
    request: Json<AddTagRequest>,
) -> Result<Status, NotFound<String>> {
    // Check if todo exists
    let todo_exists = sqlx::query!("SELECT id FROM todos WHERE id = ?", request.todo_id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Database error: {}", e)))?;

    if todo_exists.is_none() {
        return Err(NotFound("Todo not found".to_string()));
    }

    // Check if tag exists
    let tag_exists = sqlx::query!("SELECT id FROM tags WHERE id = ?", request.tag_id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Database error: {}", e)))?;

    if tag_exists.is_none() {
        return Err(NotFound("Tag not found".to_string()));
    }

    // Check if association already exists
    let existing_association = sqlx::query!(
        "SELECT id FROM todo_tags WHERE todo_id = ? AND tag_id = ?",
        request.todo_id,
        request.tag_id
    )
    .fetch_optional(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Database error: {}", e)))?;

    if existing_association.is_some() {
        return Ok(Status::Ok); // Association already exists
    }

    // Create the association
    sqlx::query!(
        "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?, ?)",
        request.todo_id,
        request.tag_id
    )
    .execute(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to create tag association: {}", e)))?;

    Ok(Status::Created)
}

pub async fn remove_tag_from_todo(
    mut db: Connection<Db>,
    todo_id: i64,
    tag_id: i64,
) -> Result<Status, NotFound<String>> {
    let result = sqlx::query!(
        "DELETE FROM todo_tags WHERE todo_id = ? AND tag_id = ?",
        todo_id,
        tag_id
    )
    .execute(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to remove tag association: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(NotFound("Tag association not found".to_string()));
    }

    Ok(Status::NoContent)
}

pub async fn get_todo_tags(
    mut db: Connection<Db>,
    todo_id: i64,
) -> Result<Json<Vec<TagResponse>>, NotFound<String>> {
    let results = sqlx::query!(
        r#"
        SELECT t.id, t.name, t.color, t.description, t.created_at, t.updated_at
        FROM tags t
        INNER JOIN todo_tags tt ON t.id = tt.tag_id
        WHERE tt.todo_id = ?
        ORDER BY t.name
        "#,
        todo_id
    )
    .fetch_all(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to fetch todo tags: {}", e)))?;

    let tags: Vec<TagResponse> = results
        .into_iter()
        .map(|row| {
            let tag = Tag {
                id: Some(row.id),
                name: row.name,
                color: row.color,
                description: row.description,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TagResponse::from(tag)
        })
        .collect();

    Ok(Json(tags))
}

pub async fn get_todos_by_tag(
    mut db: Connection<Db>,
    tag_id: i64,
) -> Result<Json<Vec<TodoWithTagsResponse>>, NotFound<String>> {
    // Check if tag exists
    let tag_exists = sqlx::query!("SELECT id, name, color, description FROM tags WHERE id = ?", tag_id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Database error: {}", e)))?;

    if tag_exists.is_none() {
        return Err(NotFound("Tag not found".to_string()));
    }

    // Get todos with this tag
    let results = sqlx::query!(
        r#"
        SELECT DISTINCT t.id, t.title, t.description, t.status, t.priority, t.due_date, t.created_at, t.updated_at
        FROM todos t
        INNER JOIN todo_tags tt ON t.id = tt.todo_id
        WHERE tt.tag_id = ?
        ORDER BY t.created_at DESC
        "#,
        tag_id
    )
    .fetch_all(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to fetch todos: {}", e)))?;

    let mut todos_with_tags = Vec::new();

    for todo_row in results {
        // Get tags for this todo
        let tag_results = sqlx::query!(
            r#"
            SELECT t.id, t.name, t.color, t.description, t.created_at, t.updated_at
            FROM tags t
            INNER JOIN todo_tags tt ON t.id = tt.tag_id
            WHERE tt.todo_id = ?
            ORDER BY t.name
            "#,
            todo_row.id
        )
        .fetch_all(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Failed to fetch todo tags: {}", e)))?;

        let tags: Vec<TagResponse> = tag_results
            .into_iter()
            .map(|tag_row| {
                let tag = Tag {
                    id: Some(tag_row.id),
                    name: tag_row.name,
                    color: tag_row.color,
                    description: tag_row.description,
                    created_at: tag_row.created_at,
                    updated_at: tag_row.updated_at,
                };
                TagResponse::from(tag)
            })
            .collect();

        let todo = Todo {
            id: Some(todo_row.id),
            title: todo_row.title,
            description: todo_row.description,
            status: todo_row.status,
            priority: todo_row.priority,
            due_date: todo_row.due_date,
            created_at: todo_row.created_at,
            updated_at: todo_row.updated_at,
        };

        let todo_response = TodoWithTagsResponse {
            id: todo.id.unwrap_or(0),
            title: todo.title,
            description: todo.description,
            status: todo.status,
            priority: todo.priority,
            due_date: todo.due_date.map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc)),
            created_at: todo.created_at
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
                .unwrap_or_else(|| chrono::Utc::now()),
            updated_at: todo.updated_at
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
                .unwrap_or_else(|| chrono::Utc::now()),
            tags,
        };

        todos_with_tags.push(todo_response);
    }

    Ok(Json(todos_with_tags))
}