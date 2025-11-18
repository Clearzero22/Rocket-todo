pub mod todo_routes;
pub mod auth_routes;

pub use todo_routes::*;
pub use auth_routes::*;

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
            crate::models::User,
            crate::models::CreateUserRequest,
            crate::models::LoginRequest,
            crate::models::UserResponse,
            crate::models::Claims
        )
    ),
    tags(
        (name = "todos", description = "Todo management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub struct ApiDoc;