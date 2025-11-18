#[macro_use]
extern crate rocket;

mod database;
mod handlers;
mod models;
mod routes;
mod telemetry;
mod auth;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rocket_db_pools::sqlx;

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    my_app_name: String,
    max_file_size: String,
}

#[get("/")]
fn index() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Welcome to Todo List API",
        "version": "1.0.0",
        "endpoints": {
            "todos": "/api/todos",
            "health": "/health",
            "docs": "/docs"
        }
    }))
}

#[get("/health")]
fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}

#[get("/live")]
fn live() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "live"
    }))
}

#[get("/config")]
fn get_config() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app_name": "Todo List API",
        "version": "1.0.0",
        "features": ["CRUD operations", "Priority levels", "Status filtering"]
    }))
}

#[get("/ready")]
async fn ready(mut db: rocket_db_pools::Connection<crate::database::Db>) -> Json<serde_json::Value> {
    // simple DB readiness check
    let ok = sqlx::query("SELECT 1")
        .fetch_one(&mut **db)
        .await
        .is_ok();

    Json(serde_json::json!({
        "status": if ok { "ready" } else { "not_ready" }
    }))
}

#[launch]
fn rocket() -> _ {
    telemetry::init_tracing();
    let openapi = routes::ApiDoc::openapi();

    rocket::build()
        .attach(telemetry::RequestTracingFairing)
        .attach(database::stage())
        .mount("/", routes![index, health, live, get_config, ready])
        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", openapi),
        )
        .mount(
            "/api",
            routes![
                // 认证路由 (公开)
                routes::auth_routes::register,
                routes::auth_routes::login,
                routes::auth_routes::logout,
                routes::auth_routes::me,
                // Todo 路由 (需要认证)
                routes::todo_routes::get_all_todos,
                routes::todo_routes::get_todo,
                routes::todo_routes::get_todos_by_status,
                routes::todo_routes::get_todos_by_priority,
                routes::todo_routes::create_todo,
                routes::todo_routes::update_todo,
                routes::todo_routes::delete_todo
            ],
        )
}
