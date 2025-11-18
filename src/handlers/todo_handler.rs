use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use crate::database::{Db, DbResult};
use crate::models::{
    CreateTodoRequest, Priority, Status as TodoStatus, Todo, TodoResponse, UpdateTodoRequest,
};
use rocket_db_pools::Connection;

pub async fn create_todo(
    mut db: Connection<Db>,
    request: Json<CreateTodoRequest>,
) -> DbResult<Json<TodoResponse>> {
    let priority = request.priority.as_ref().unwrap_or(&Priority::Medium);
    let status = request.status.as_ref().unwrap_or(&TodoStatus::Pending);

    let status_str = status.as_str();
    let priority_str = priority.as_str();

    let result = sqlx::query!(
        "INSERT INTO todos (title, description, status, priority, due_date) VALUES (?, ?, ?, ?, ?)",
        request.title,
        request.description,
        status_str,
        priority_str,
        request.due_date
    )
    .execute(&mut **db)
    .await?;

    let id = result.last_insert_rowid();

    // Get the created record
    let created_record = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await?;

    let todo = Todo {
        id: Some(created_record.id),
        title: created_record.title,
        description: created_record.description,
        status: created_record.status,
        priority: created_record.priority,
        due_date: created_record.due_date,
        created_at: created_record.created_at,
        updated_at: created_record.updated_at,
    };

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn get_todo(
    mut db: Connection<Db>,
    id: i64,
) -> Result<Json<TodoResponse>, NotFound<String>> {
    let result = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Todo not found: {}", e)))?;

    let todo = Todo {
        id: Some(result.id),
        title: result.title,
        description: result.description,
        status: result.status,
        priority: result.priority,
        due_date: result.due_date,
        created_at: result.created_at,
        updated_at: result.updated_at,
    };

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn get_all_todos(mut db: Connection<Db>) -> DbResult<Json<Vec<TodoResponse>>> {
    let results = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos ORDER BY created_at DESC"
    )
    .fetch_all(&mut **db)
    .await?;

    let todos: Vec<TodoResponse> = results
        .into_iter()
        .map(|row| {
            let todo = Todo {
                id: Some(row.id),
                title: row.title,
                description: row.description,
                status: row.status,
                priority: row.priority,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TodoResponse::from(todo)
        })
        .collect();

    Ok(Json(todos))
}

pub async fn update_todo(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, NotFound<String>> {
    // First, get the existing todo
    let existing = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Todo not found: {}", e)))?;

    // Build update query dynamically
    let mut update_fields = Vec::new();
    let mut values: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();

    if let Some(title) = &request.title {
        update_fields.push("title = ?");
        values.push(Box::new(title.clone()));
    }
    if let Some(description) = &request.description {
        update_fields.push("description = ?");
        values.push(Box::new(description.clone()));
    }
    if let Some(status) = &request.status {
        update_fields.push("status = ?");
        values.push(Box::new(status.as_str().to_string()));
    }
    if let Some(priority) = &request.priority {
        update_fields.push("priority = ?");
        values.push(Box::new(priority.as_str().to_string()));
    }
    if let Some(due_date) = &request.due_date {
        update_fields.push("due_date = ?");
        values.push(Box::new(*due_date));
    }

    if update_fields.is_empty() {
        // No fields to update, return existing todo
        let todo = Todo {
            id: Some(existing.id),
            title: existing.title,
            description: existing.description,
            status: existing.status,
            priority: existing.priority,
            due_date: existing.due_date,
            created_at: existing.created_at,
            updated_at: existing.updated_at,
        };
        return Ok(Json(TodoResponse::from(todo)));
    }

    update_fields.push("updated_at = CURRENT_TIMESTAMP");

    let query = format!("UPDATE todos SET {} WHERE id = ?", update_fields.join(", "));

    // Execute the update
    let result = sqlx::query(&query)
        .bind(id)
        .execute(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Failed to update todo: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(NotFound("Todo not found".to_string()));
    }

    // Get the updated record
    let updated_record = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|e| NotFound(format!("Failed to fetch updated todo: {}", e)))?;

    let result = Todo {
        id: Some(updated_record.id),
        title: updated_record.title,
        description: updated_record.description,
        status: updated_record.status,
        priority: updated_record.priority,
        due_date: updated_record.due_date,
        created_at: updated_record.created_at,
        updated_at: updated_record.updated_at,
    };

    Ok(Json(TodoResponse::from(result)))
}

pub async fn delete_todo(mut db: Connection<Db>, id: i64) -> Result<Status, NotFound<String>> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&mut **db)
        .await
        .map_err(|e| NotFound(format!("Failed to delete todo: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(NotFound("Todo not found".to_string()));
    }

    Ok(Status::NoContent)
}

pub async fn get_todos_by_status(
    mut db: Connection<Db>,
    status: String,
) -> DbResult<Json<Vec<TodoResponse>>> {
    let results = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE status = ? ORDER BY created_at DESC",
        status
    )
    .fetch_all(&mut **db)
    .await?;

    let todos: Vec<TodoResponse> = results
        .into_iter()
        .map(|row| {
            let todo = Todo {
                id: row.id,
                title: row.title,
                description: row.description,
                status: row.status,
                priority: row.priority,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TodoResponse::from(todo)
        })
        .collect();

    Ok(Json(todos))
}

pub async fn get_todos_by_priority(
    mut db: Connection<Db>,
    priority: String,
) -> DbResult<Json<Vec<TodoResponse>>> {
    let results = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE priority = ? ORDER BY created_at DESC",
        priority
    )
    .fetch_all(&mut **db)
    .await?;

    let todos: Vec<TodoResponse> = results
        .into_iter()
        .map(|row| {
            let todo = Todo {
                id: row.id,
                title: row.title,
                description: row.description,
                status: row.status,
                priority: row.priority,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TodoResponse::from(todo)
        })
        .collect();

    Ok(Json(todos))
}

// New functions for due date functionality

pub async fn get_overdue_todos(mut db: Connection<Db>) -> DbResult<Json<Vec<TodoResponse>>> {
    let now = chrono::Utc::now().naive_utc();
    let results = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE due_date < ? AND status != 'completed' ORDER BY due_date ASC",
        now
    )
    .fetch_all(&mut **db)
    .await?;

    let todos: Vec<TodoResponse> = results
        .into_iter()
        .map(|row| {
            let todo = Todo {
                id: row.id,
                title: row.title,
                description: row.description,
                status: row.status,
                priority: row.priority,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TodoResponse::from(todo)
        })
        .collect();

    Ok(Json(todos))
}

pub async fn get_upcoming_todos(
    mut db: Connection<Db>,
    days: i64,
) -> DbResult<Json<Vec<TodoResponse>>> {
    let now = chrono::Utc::now().naive_utc();
    let future_date = now + chrono::Duration::days(days);

    let results = sqlx::query!(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE due_date >= ? AND due_date <= ? AND status != 'completed' ORDER BY due_date ASC",
        now,
        future_date
    )
    .fetch_all(&mut **db)
    .await?;

    let todos: Vec<TodoResponse> = results
        .into_iter()
        .map(|row| {
            let todo = Todo {
                id: row.id,
                title: row.title,
                description: row.description,
                status: row.status,
                priority: row.priority,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            TodoResponse::from(todo)
        })
        .collect();

    Ok(Json(todos))
}
