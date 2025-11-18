pub mod todo_routes;
pub mod auth_routes;
pub mod tag_routes;
pub mod subtask_routes;

pub use todo_routes::*;
pub use auth_routes::*;
pub use tag_routes::*;
pub use subtask_routes::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todo_routes::get_all_todos,
        crate::routes::todo_routes::get_todo,
        crate::routes::todo_routes::get_todos_by_status,
        crate::routes::todo_routes::get_todos_by_priority,
        crate::routes::todo_routes::create_todo,
        crate::routes::todo_routes::update_todo,
        crate::routes::todo_routes::delete_todo,
        crate::routes::tag_routes::get_all_tags,
        crate::routes::tag_routes::get_tag,
        crate::routes::tag_routes::create_tag,
        crate::routes::tag_routes::update_tag,
        crate::routes::tag_routes::delete_tag,
        crate::routes::tag_routes::add_tag_to_todo,
        crate::routes::tag_routes::remove_tag_from_todo,
        crate::routes::tag_routes::get_todo_tags,
        crate::routes::tag_routes::get_todos_by_tag,
        crate::routes::subtask_routes::get_subtasks_by_todo,
        crate::routes::subtask_routes::get_todo_with_subtasks,
        crate::routes::subtask_routes::get_subtask,
        crate::routes::subtask_routes::create_subtask,
        crate::routes::subtask_routes::create_subtask_for_todo,
        crate::routes::subtask_routes::update_subtask,
        crate::routes::subtask_routes::delete_subtask,
        crate::routes::subtask_routes::get_subtasks_by_status,
        crate::routes::subtask_routes::get_overdue_subtasks,
        crate::routes::subtask_routes::reorder_subtasks,
        crate::routes::auth_routes::register,
        crate::routes::auth_routes::login,
        crate::routes::auth_routes::logout,
        crate::routes::auth_routes::me
    ),
    components(
        schemas(
            crate::models::Todo,
            crate::models::TodoResponse,
            crate::models::CreateTodoRequest,
            crate::models::UpdateTodoRequest,
            crate::models::Priority,
            crate::models::Status,
            crate::models::Tag,
            crate::models::TagResponse,
            crate::models::CreateTagRequest,
            crate::models::UpdateTagRequest,
            crate::models::TodoTag,
            crate::models::AddTagRequest,
            crate::models::TodoWithTagsResponse,
            crate::models::Subtask,
            crate::models::SubtaskResponse,
            crate::models::CreateSubtaskRequest,
            crate::models::UpdateSubtaskRequest,
            crate::models::TodoWithSubtasksResponse,
            crate::models::ReorderSubtasksRequest,
            crate::models::User,
            crate::models::CreateUserRequest,
            crate::models::LoginRequest,
            crate::models::UserResponse,
            crate::models::Claims
        )
    ),
    tags(
        (name = "todos", description = "Todo management endpoints"),
        (name = "tags", description = "Tag management endpoints"),
        (name = "subtasks", description = "Subtask management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub struct ApiDoc;