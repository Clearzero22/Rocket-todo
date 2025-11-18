use rocket::delete;
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::put;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::database::Db;
use crate::handlers::tag_handler;
use crate::models::{CreateTagRequest, TagResponse, UpdateTagRequest, AddTagRequest, TodoWithTagsResponse};
use crate::auth::jwt::JwtAuth;
use rocket_db_pools::Connection;

// Tag CRUD routes

#[utoipa::path(get, path = "/api/tags", tag = "tags", responses(
    (status = 200, description = "List all tags", body = [TagResponse])
))]
#[get("/tags")]
pub async fn get_all_tags(
    mut db: Connection<Db>,
    auth: JwtAuth,
) -> Result<Json<Vec<TagResponse>>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::get_all_tags(db).await {
        Ok(tags) => Ok(tags),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to fetch tags",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/tags/<id>", tag = "tags", responses(
    (status = 200, description = "Get tag by ID", body = TagResponse),
    (status = 404, description = "Tag not found")
))]
#[get("/tags/<id>")]
pub async fn get_tag(
    mut db: Connection<Db>,
    id: i64,
    auth: JwtAuth,
) -> Result<Json<TagResponse>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::get_tag(db, id).await {
        Ok(tag) => Ok(tag),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Tag not found",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(post, path = "/api/tags", tag = "tags", responses(
    (status = 201, description = "Create new tag", body = TagResponse),
    (status = 400, description = "Invalid request")
))]
#[post("/tags", format = "json", data = "<request>")]
pub async fn create_tag(
    mut db: Connection<Db>,
    request: Json<CreateTagRequest>,
    auth: JwtAuth,
) -> Result<Json<TagResponse>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::create_tag(db, request).await {
        Ok(tag) => Ok(tag),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({
                "error": "Failed to create tag",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(put, path = "/api/tags/<id>", tag = "tags", responses(
    (status = 200, description = "Update tag", body = TagResponse),
    (status = 404, description = "Tag not found")
))]
#[put("/tags/<id>", format = "json", data = "<request>")]
pub async fn update_tag(
    mut db: Connection<Db>,
    id: i64,
    request: Json<UpdateTagRequest>,
    auth: JwtAuth,
) -> Result<Json<TagResponse>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::update_tag(db, id, request).await {
        Ok(tag) => Ok(tag),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Tag not found or update failed",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(delete, path = "/api/tags/<id>", tag = "tags", responses(
    (status = 204, description = "Delete tag successfully"),
    (status = 404, description = "Tag not found")
))]
#[delete("/tags/<id>")]
pub async fn delete_tag(
    mut db: Connection<Db>,
    id: i64,
    auth: JwtAuth,
) -> Result<Status, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::delete_tag(db, id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Tag not found",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

// Todo-Tag association routes

#[utoipa::path(post, path = "/api/todos/<todo_id>/tags", tag = "todos", responses(
    (status = 201, description = "Add tag to todo"),
    (status = 404, description = "Todo or tag not found")
))]
#[post("/todos/<todo_id>/tags", format = "json", data = "<request>")]
pub async fn add_tag_to_todo(
    mut db: Connection<Db>,
    todo_id: i64,
    request: Json<AddTagRequest>,
    auth: JwtAuth,
) -> Result<Status, status::Custom<Json<serde_json::Value>>> {
    let add_request = AddTagRequest {
        todo_id,
        tag_id: request.tag_id,
    };

    match tag_handler::add_tag_to_todo(db, Json(add_request)).await {
        Ok(status) => Ok(status),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Failed to add tag to todo",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(delete, path = "/api/todos/<todo_id>/tags/<tag_id>", tag = "todos", responses(
    (status = 204, description = "Remove tag from todo"),
    (status = 404, description = "Association not found")
))]
#[delete("/todos/<todo_id>/tags/<tag_id>")]
pub async fn remove_tag_from_todo(
    mut db: Connection<Db>,
    todo_id: i64,
    tag_id: i64,
    auth: JwtAuth,
) -> Result<Status, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::remove_tag_from_todo(db, todo_id, tag_id).await {
        Ok(status) => Ok(status),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Failed to remove tag from todo",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/todos/<todo_id>/tags", tag = "todos", responses(
    (status = 200, description = "Get todo tags", body = [TagResponse])
))]
#[get("/todos/<todo_id>/tags", rank = 1)]
pub async fn get_todo_tags(
    mut db: Connection<Db>,
    todo_id: i64,
    auth: JwtAuth,
) -> Result<Json<Vec<TagResponse>>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::get_todo_tags(db, todo_id).await {
        Ok(tags) => Ok(tags),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Failed to fetch todo tags",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(get, path = "/api/tags/<tag_id>/todos", tag = "tags", responses(
    (status = 200, description = "Get todos by tag", body = [TodoWithTagsResponse])
))]
#[get("/tags/<tag_id>/todos")]
pub async fn get_todos_by_tag(
    mut db: Connection<Db>,
    tag_id: i64,
    auth: JwtAuth,
) -> Result<Json<Vec<TodoWithTagsResponse>>, status::Custom<Json<serde_json::Value>>> {
    match tag_handler::get_todos_by_tag(db, tag_id).await {
        Ok(todos) => Ok(todos),
        Err(e) => Err(status::Custom(
            Status::NotFound,
            Json(serde_json::json!({
                "error": "Failed to fetch todos by tag",
                "message": format!("{:?}", e)
            })),
        )),
    }
}