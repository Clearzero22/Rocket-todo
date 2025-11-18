use rocket::delete;
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::put;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::database::Db;
use crate::handlers::subtask_handler;
use crate::models::{
    CreateSubtaskRequest, SubtaskResponse, UpdateSubtaskRequest,
    TodoWithSubtasksResponse, ReorderSubtasksRequest
};
use crate::auth::jwt::JwtAuth;
use rocket_db_pools::Connection;

// Subtask CRUD routes

#[utoipa::path(get, path = "/api/todos/{parent_todo_id}/subtasks", tag = "subtasks", params(
    ("parent_todo_id" = i64, Path, description = "Parent todo ID")
), responses(
    (status = 200, description = "List all subtasks for a todo", body = [SubtaskResponse]),
    (status = 404, description = "Parent todo not found")
))]
#[get("/todos/<parent_todo_id>/subtasks")]
pub async fn get_subtasks_by_todo(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    auth: JwtAuth,
) -> Result<Json<Vec<SubtaskResponse>>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::get_subtasks_by_todo(db, parent_todo_id).await {
        Ok(subtasks) => Ok(subtasks),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch subtasks",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/{todo_id}/with-subtasks", tag = "todos", params(
    ("todo_id" = i64, Path, description = "Todo ID")
), responses(
    (status = 200, description = "Get todo with all its subtasks", body = TodoWithSubtasksResponse),
    (status = 404, description = "Todo not found")
))]
#[get("/todos/<todo_id>/with-subtasks")]
pub async fn get_todo_with_subtasks(
    mut db: Connection<Db>,
    todo_id: i64,
    auth: JwtAuth,
) -> Result<Json<TodoWithSubtasksResponse>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::get_todo_with_subtasks(db, todo_id).await {
        Ok(todo_with_subtasks) => Ok(todo_with_subtasks),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch todo with subtasks",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/subtasks/{id}", tag = "subtasks", params(
    ("id" = i64, Path, description = "Subtask ID")
), responses(
    (status = 200, description = "Get subtask by ID", body = SubtaskResponse),
    (status = 404, description = "Subtask not found")
))]
#[get("/subtasks/<id>")]
pub async fn get_subtask(
    mut db: Connection<Db>,
    id: i64,
    auth: JwtAuth,
) -> Result<Json<SubtaskResponse>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::get_subtask(db, id).await {
        Ok(subtask) => Ok(subtask),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Subtask not found",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(post, path = "/api/subtasks", tag = "subtasks", request_body = CreateSubtaskRequest, responses(
    (status = 201, description = "Create new subtask", body = SubtaskResponse),
    (status = 400, description = "Invalid request"),
    (status = 404, description = "Parent todo not found")
))]
#[post("/subtasks", data = "<request>")]
pub async fn create_subtask(
    mut db: Connection<Db>,
    request: Json<CreateSubtaskRequest>,
    auth: JwtAuth,
) -> Result<Json<SubtaskResponse>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::create_subtask(db, request).await {
        Ok(subtask) => Ok(subtask),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to create subtask",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(post, path = "/api/todos/{parent_todo_id}/subtasks", tag = "subtasks", params(
    ("parent_todo_id" = i64, Path, description = "Parent todo ID")
), request_body = CreateSubtaskRequest, responses(
    (status = 201, description = "Create new subtask for specific todo", body = SubtaskResponse),
    (status = 400, description = "Invalid request"),
    (status = 404, description = "Parent todo not found")
))]
#[post("/todos/<parent_todo_id>/subtasks", data = "<request>")]
pub async fn create_subtask_for_todo(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    mut request: Json<CreateSubtaskRequest>,
    auth: JwtAuth,
) -> Result<Json<SubtaskResponse>, status::Custom<Json<serde_json::Value>>> {
    // Override parent_todo_id in request with path parameter
    request.parent_todo_id = parent_todo_id;

    match subtask_handler::create_subtask(db, request).await {
        Ok(subtask) => Ok(subtask),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to create subtask",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(put, path = "/api/subtasks/{id}", tag = "subtasks", params(
    ("id" = i64, Path, description = "Subtask ID")
), request_body = UpdateSubtaskRequest, responses(
    (status = 200, description = "Update subtask", body = SubtaskResponse),
    (status = 404, description = "Subtask not found"),
    (status = 400, description = "Invalid request")
))]
#[put("/subtasks/<id>", data = "<request>")]
pub async fn update_subtask(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateSubtaskRequest>,
    auth: JwtAuth,
) -> Result<Json<SubtaskResponse>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::update_subtask(db, id, request).await {
        Ok(subtask) => Ok(subtask),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to update subtask",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(delete, path = "/api/subtasks/{id}", tag = "subtasks", params(
    ("id" = i64, Path, description = "Subtask ID")
), responses(
    (status = 204, description = "Delete subtask successfully"),
    (status = 404, description = "Subtask not found")
))]
#[delete("/subtasks/<id>")]
pub async fn delete_subtask(
    mut db: Connection<Db>,
    id: i64,
    auth: JwtAuth,
) -> Result<status::NoContent, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::delete_subtask(db, id).await {
        Ok(_) => Ok(status::NoContent),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Subtask not found",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

// Advanced subtask operations

#[utoipa::path(get, path = "/api/todos/{parent_todo_id}/subtasks/status/{status}", tag = "subtasks", params(
    ("parent_todo_id" = i64, Path, description = "Parent todo ID"),
    ("status" = String, Path, description = "pending | in_progress | completed")
), responses(
    (status = 200, description = "Get subtasks by status", body = [SubtaskResponse]),
    (status = 404, description = "Parent todo not found"),
    (status = 400, description = "Invalid status")
))]
#[get("/todos/<parent_todo_id>/subtasks/status/<status>")]
pub async fn get_subtasks_by_status(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    status: String,
    auth: JwtAuth,
) -> Result<Json<Vec<SubtaskResponse>>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::get_subtasks_by_status(db, parent_todo_id, status).await {
        Ok(subtasks) => Ok(subtasks),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to fetch subtasks by status",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/{parent_todo_id}/subtasks/overdue", tag = "subtasks", params(
    ("parent_todo_id" = i64, Path, description = "Parent todo ID")
), responses(
    (status = 200, description = "Get overdue subtasks", body = [SubtaskResponse]),
    (status = 404, description = "Parent todo not found")
))]
#[get("/todos/<parent_todo_id>/subtasks/overdue")]
pub async fn get_overdue_subtasks(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    auth: JwtAuth,
) -> Result<Json<Vec<SubtaskResponse>>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::get_overdue_subtasks(db, parent_todo_id).await {
        Ok(subtasks) => Ok(subtasks),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch overdue subtasks",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(put, path = "/api/todos/{parent_todo_id}/subtasks/reorder", tag = "subtasks", params(
    ("parent_todo_id" = i64, Path, description = "Parent todo ID")
), request_body = ReorderSubtasksRequest, responses(
    (status = 200, description = "Reorder subtasks", body = [SubtaskResponse]),
    (status = 404, description = "Parent todo not found"),
    (status = 400, description = "Invalid request")
))]
#[put("/todos/<parent_todo_id>/subtasks/reorder", data = "<request>")]
pub async fn reorder_subtasks(
    mut db: Connection<Db>,
    parent_todo_id: i64,
    request: Json<ReorderSubtasksRequest>,
    auth: JwtAuth,
) -> Result<Json<Vec<SubtaskResponse>>, status::Custom<Json<serde_json::Value>>> {
    match subtask_handler::reorder_subtasks(db, parent_todo_id, request).await {
        Ok(subtasks) => Ok(subtasks),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to reorder subtasks",
                "message": format!("{:?}", e)
            })),
        )),
    }
}