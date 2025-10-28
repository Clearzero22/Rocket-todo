use rocket::delete;
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::put;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::database::Db;
use crate::handlers::todo_handler;
use crate::models::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};
use rocket_db_pools::Connection;

#[utoipa::path(get, path = "/api/todos", tag = "todos", responses(
    (status = 200, description = "List todos", body = [TodoResponse])
))]
#[get("/todos")]
pub async fn get_all_todos(
    mut db: Connection<Db>,
) -> Result<Json<Vec<TodoResponse>>, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::get_all_todos(db).await {
        Ok(todos) => Ok(todos),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch todos",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/{id}", tag = "todos", params(
    ("id" = i64, Path, description = "Todo id")
), responses(
    (status = 200, description = "Get todo", body = TodoResponse),
    (status = 404, description = "Not found")
))]
#[get("/todos/<id>")]
pub async fn get_todo(
    mut db: Connection<Db>,
    id: i64,
) -> Result<Json<TodoResponse>, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::get_todo(db, id).await {
        Ok(todo) => Ok(todo),
        Err(not_found) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Todo not found",
                "message": not_found.0
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/status/{status}", tag = "todos", params(
    ("status" = String, Path, description = "pending | in_progress | completed")
), responses(
    (status = 200, description = "List by status", body = [TodoResponse])
))]
#[get("/todos/status/<status>")]
pub async fn get_todos_by_status(
    mut db: Connection<Db>,
    status: String,
) -> Result<Json<Vec<TodoResponse>>, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::get_todos_by_status(db, status).await {
        Ok(todos) => Ok(todos),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch todos",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(post, path = "/api/todos", tag = "todos", request_body = CreateTodoRequest, responses(
    (status = 201, description = "Created", body = TodoResponse)
))]
#[post("/todos", data = "<request>")]
pub async fn create_todo(
    mut db: Connection<Db>,
    request: Json<CreateTodoRequest>,
) -> Result<status::Created<Json<TodoResponse>>, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::create_todo(db, request).await {
        Ok(todo) => Ok(status::Created::new("/todos").body(todo)),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Failed to create todo",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(put, path = "/api/todos/{id}", tag = "todos", request_body = UpdateTodoRequest, params(
    ("id" = i64, Path,)
), responses(
    (status = 200, description = "Updated", body = TodoResponse),
    (status = 404, description = "Not found")
))]
#[put("/todos/<id>", data = "<request>")]
pub async fn update_todo(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::update_todo(db, id, request).await {
        Ok(todo) => Ok(todo),
        Err(not_found) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Todo not found",
                "message": not_found.0
            })),
        )),
    }
}

#[utoipa::path(delete, path = "/api/todos/{id}", tag = "todos", params(
    ("id" = i64, Path,)
), responses(
    (status = 204, description = "Deleted"),
    (status = 404, description = "Not found")
))]
#[delete("/todos/<id>")]
pub async fn delete_todo(
    mut db: Connection<Db>,
    id: i64,
) -> Result<status::NoContent, status::Custom<Json<serde_json::Value>>> {
    match todo_handler::delete_todo(db, id).await {
        Ok(_) => Ok(status::NoContent),
        Err(not_found) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Todo not found",
                "message": not_found.0
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/priority/{priority}", tag = "todos", params(
    ("priority" = String, Path, description = "low | medium | high")
), responses(
    (status = 200, description = "List by priority", body = [TodoResponse])
))]
#[get("/todos/priority/<priority>")]
pub async fn get_todos_by_priority(
    mut db: Connection<Db>,
    priority: String,
) -> Result<Json<Vec<TodoResponse>>, status::Custom<Json<serde_json::Value>>> {
    // Validate priority
    if !["low", "medium", "high"].contains(&priority.to_lowercase().as_str()) {
        return Err(status::Custom(
            Status::BadRequest,
            Json(serde_json::json!({
                "error": "Invalid priority",
                "message": "Priority must be 'low', 'medium', or 'high'"
            })),
        ));
    }

    match todo_handler::get_todos_by_priority(db, priority).await {
        Ok(todos) => Ok(todos),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch todos",
                "message": format!("{:?}", e)
            })),
        )),
    }
}
