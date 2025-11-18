use rocket::delete;
use rocket::get;
use rocket::http::CookieJar;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::auth::jwt::JwtAuth;
use crate::database::Db;
use crate::handlers::auth_handler;
use crate::models::user::{CreateUserRequest, LoginRequest};
use rocket_db_pools::Connection;
use utoipa::path;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 409, description = "User already exists")
    )
)]
#[post("/auth/register", data = "<request>")]
pub async fn register(
    db: Connection<Db>,
    request: Json<CreateUserRequest>,
) -> Result<status::Created<Json<serde_json::Value>>, status::Custom<Json<serde_json::Value>>> {
    auth_handler::register(db, request).await
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful"),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/auth/login", data = "<request>")]
pub async fn login(
    db: Connection<Db>,
    request: Json<LoginRequest>,
    cookies: &CookieJar<'_>,
) -> Result<Json<serde_json::Value>, status::Custom<Json<serde_json::Value>>> {
    auth_handler::login(db, request, cookies).await
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful")
    )
)]
#[post("/auth/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Json<serde_json::Value> {
    auth_handler::logout(cookies).await
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Get current user info"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
#[get("/auth/me")]
pub async fn me(auth: JwtAuth) -> Json<serde_json::Value> {
    auth_handler::me(auth).await
}