pub mod todo_routes;

pub use todo_routes::*;

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
        crate::routes::todo_routes::delete_todo
    ),
    components(
        schemas(
            crate::models::Todo,
            crate::models::TodoResponse,
            crate::models::CreateTodoRequest,
            crate::models::UpdateTodoRequest,
            crate::models::Priority,
            crate::models::Status
        )
    ),
    tags(
        (name = "todos", description = "Todo management endpoints")
    )
)]
pub struct ApiDoc;