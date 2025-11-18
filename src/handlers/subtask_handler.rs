use rocket::response::status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::database::{Db, DbResult};
use crate::models::{
    CreateSubtaskRequest, Subtask, SubtaskResponse, UpdateSubtaskRequest,
    TodoWithSubtasksResponse, ReorderSubtasksRequest
};

// Subtask CRUD operations

pub async fn create_subtask(
    mut db: Connection<Db>,
    request: Json<CreateSubtaskRequest>,
) -> DbResult<Json<SubtaskResponse>> {
    // Validate request
    if let Err(e) = request.validate() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Validation error: {}", e)))));
    }

    // Check if parent todo exists
    let parent_exists = sqlx::query!("SELECT id FROM todos WHERE id = ?", request.parent_todo_id)
        .fetch_optional(&mut **db)
        .await?;

    if parent_exists.is_none() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Parent todo not found"))));
    }

    // Get next order index if not provided
    let order_index = match request.order_index {
        Some(order) => order,
        None => {
            let result = sqlx::query!("SELECT COALESCE(MAX(order_index), -1) + 1 as next_order FROM subtasks WHERE parent_todo_id = ?", request.parent_todo_id)
                .fetch_one(&mut **db)
                .await?;

            result.next_order.into()
        }
    };

    let priority_str = request.priority.as_ref().map(|s| s.as_str()).unwrap_or("medium");

    let result = sqlx::query!(
        "INSERT INTO subtasks (parent_todo_id, title, description, status, priority, due_date, order_index) VALUES (?, ?, ?, ?, ?, ?, ?)",
        request.parent_todo_id,
        request.title,
        request.description,
        "pending", // Always start with pending status
        priority_str,
        request.due_date,
        order_index
    )
    .execute(&mut **db)
    .await?;

    let id = result.last_insert_rowid();

    get_subtask(db, id).await
}

pub async fn get_subtask(
    mut db: Connection<Db>,
    id: i64,
) -> DbResult<Json<SubtaskResponse>> {
    let subtask = sqlx::query_as!(
        Subtask,
        "SELECT id, parent_todo_id, title, description, status, priority, due_date, order_index, created_at, updated_at FROM subtasks WHERE id = ?",
        id
    )
    .fetch_optional(&mut **db)
    .await?;

    match subtask {
        Some(subtask) => Ok(Json(SubtaskResponse::from(subtask))),
        None => Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Subtask not found")))),
    }
}

pub async fn get_subtasks_by_todo(
    mut db: Connection<Db>,
    parent_todo_id: i64,
) -> DbResult<Json<Vec<SubtaskResponse>>> {
    // Check if parent todo exists
    let parent_exists = sqlx::query!("SELECT id FROM todos WHERE id = ?", parent_todo_id)
        .fetch_optional(&mut **db)
        .await?;

    if parent_exists.is_none() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Parent todo not found"))));
    }

    let subtasks = sqlx::query_as!(
        Subtask,
        "SELECT id, parent_todo_id, title, description, status, priority, due_date, order_index, created_at, updated_at FROM subtasks WHERE parent_todo_id = ? ORDER BY order_index ASC, created_at ASC",
        parent_todo_id
    )
    .fetch_all(&mut **db)
    .await?;

    let subtask_responses: Vec<SubtaskResponse> = subtasks
        .into_iter()
        .map(SubtaskResponse::from)
        .collect();

    Ok(Json(subtask_responses))
}

pub async fn get_todo_with_subtasks(
    mut db: Connection<Db>,
    todo_id: i64,
) -> DbResult<Json<TodoWithSubtasksResponse>> {
    // Get the main todo
    let todo = sqlx::query_as!(
        crate::models::Todo,
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM todos WHERE id = ?",
        todo_id
    )
    .fetch_optional(&mut **db)
    .await?;

    let todo = match todo {
        Some(todo) => todo,
        None => return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Todo not found")))),
    };

    // Get subtasks
    let subtasks = sqlx::query_as!(
        Subtask,
        "SELECT id, parent_todo_id, title, description, status, priority, due_date, order_index, created_at, updated_at FROM subtasks WHERE parent_todo_id = ? ORDER BY order_index ASC, created_at ASC",
        todo_id
    )
    .fetch_all(&mut **db)
    .await?;

    let subtask_responses: Vec<SubtaskResponse> = subtasks
        .into_iter()
        .map(SubtaskResponse::from)
        .collect();

    let todo_response = TodoWithSubtasksResponse {
        id: todo.id.unwrap_or(0),
        title: todo.title,
        description: todo.description,
        status: todo.status,
        priority: todo.priority,
        due_date: todo.due_date
            .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc)),
        created_at: todo
            .created_at
            .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now()),
        updated_at: todo
            .updated_at
            .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now()),
        subtasks: subtask_responses,
    };

    Ok(Json(todo_response))
}

pub async fn update_subtask(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateSubtaskRequest>,
) -> DbResult<Json<SubtaskResponse>> {
    // Validate request
    if let Err(e) = request.validate() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Validation error: {}", e)))));
    }

    // Check if subtask exists
    let existing = sqlx::query!("SELECT id FROM subtasks WHERE id = ?", id)
        .fetch_optional(&mut **db)
        .await?;

    if existing.is_none() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Subtask not found"))));
    }

    // Update fields individually
    if let Some(ref title) = request.title {
        sqlx::query!("UPDATE subtasks SET title = ? WHERE id = ?", title, id)
            .execute(&mut **db)
            .await?;
    }
    if let Some(ref description) = request.description {
        sqlx::query!("UPDATE subtasks SET description = ? WHERE id = ?", description, id)
            .execute(&mut **db)
            .await?;
    }
    if let Some(ref status) = request.status {
        sqlx::query!("UPDATE subtasks SET status = ? WHERE id = ?", status, id)
            .execute(&mut **db)
            .await?;
    }
    if let Some(ref priority) = request.priority {
        sqlx::query!("UPDATE subtasks SET priority = ? WHERE id = ?", priority, id)
            .execute(&mut **db)
            .await?;
    }
    if let Some(ref due_date) = request.due_date {
        sqlx::query!("UPDATE subtasks SET due_date = ? WHERE id = ?", due_date, id)
            .execute(&mut **db)
            .await?;
    }
    if let Some(order_index) = request.order_index {
        sqlx::query!("UPDATE subtasks SET order_index = ? WHERE id = ?", order_index, id)
            .execute(&mut **db)
            .await?;
    }

    get_subtask(db, id).await
}

pub async fn delete_subtask(
    mut db: Connection<Db>,
    id: i64,
) -> DbResult<status::NoContent> {
    let result = sqlx::query!("DELETE FROM subtasks WHERE id = ?", id)
        .execute(&mut **db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Subtask not found"))));
    }

    Ok(status::NoContent)
}

pub async fn reorder_subtasks(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    request: Json<ReorderSubtasksRequest>,
) -> DbResult<Json<Vec<SubtaskResponse>>> {
    // Validate request
    if let Err(e) = request.validate() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Validation error: {}", e)))));
    }

    // Check if parent todo exists
    let parent_exists = sqlx::query!("SELECT id FROM todos WHERE id = ?", parent_todo_id)
        .fetch_optional(&mut **db)
        .await?;

    if parent_exists.is_none() {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Parent todo not found"))));
    }

    // Begin transaction
    let mut tx = db.begin().await?;

    // Update order indices
    for (index, subtask_id) in request.subtask_ids.iter().enumerate() {
        let order_index = index as i64;
        let result = sqlx::query!(
            "UPDATE subtasks SET order_index = ? WHERE id = ? AND parent_todo_id = ?",
            order_index,
            subtask_id,
            parent_todo_id
        )
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 0 {
            tx.rollback().await?;
            return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Subtask with ID {} not found or doesn't belong to specified todo", subtask_id)))));
        }
    }

    // Commit transaction
    tx.commit().await?;

    // Return updated subtasks
    get_subtasks_by_todo(db, parent_todo_id).await
}

pub async fn get_subtasks_by_status(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    status: String,
) -> DbResult<Json<Vec<SubtaskResponse>>> {
    if !["pending", "in_progress", "completed"].contains(&status.as_str()) {
        return Err(rocket::response::Debug(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid status"))));
    }

    let subtasks = sqlx::query_as!(
        Subtask,
        "SELECT id, parent_todo_id, title, description, status, priority, due_date, order_index, created_at, updated_at FROM subtasks WHERE parent_todo_id = ? AND status = ? ORDER BY order_index ASC, created_at ASC",
        parent_todo_id,
        status
    )
    .fetch_all(&mut **db)
    .await?;

    let subtask_responses: Vec<SubtaskResponse> = subtasks
        .into_iter()
        .map(SubtaskResponse::from)
        .collect();

    Ok(Json(subtask_responses))
}

pub async fn get_overdue_subtasks(
    mut db: Connection<Db>,
    parent_todo_id: i64,
) -> DbResult<Json<Vec<SubtaskResponse>>> {
    let now = chrono::Utc::now().naive_utc();

    let subtasks = sqlx::query_as!(
        Subtask,
        "SELECT id, parent_todo_id, title, description, status, priority, due_date, order_index, created_at, updated_at FROM subtasks WHERE parent_todo_id = ? AND due_date < ? AND status != 'completed' ORDER BY due_date ASC",
        parent_todo_id,
        now
    )
    .fetch_all(&mut **db)
    .await?;

    let subtask_responses: Vec<SubtaskResponse> = subtasks
        .into_iter()
        .map(SubtaskResponse::from)
        .collect();

    Ok(Json(subtask_responses))
}