use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::http::{Cookie, SameSite, Status};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::database::Db;
use crate::models::user::{CreateUserRequest, LoginRequest, User};
use crate::auth::jwt::create_token;
use rocket::http::CookieJar;

pub async fn register(
    mut db: Connection<Db>,
    request: Json<CreateUserRequest>,
) -> Result<status::Created<Json<serde_json::Value>>, status::Custom<Json<serde_json::Value>>> {
    // Check if user exists
    let existing_user = sqlx::query!("SELECT id FROM users WHERE email = ? OR username = ?",
        request.email, request.username)
        .fetch_optional(&mut **db)
        .await;

    if existing_user.is_ok() && existing_user.unwrap().is_some() {
        return Err(status::Custom(
            Status::Conflict,
            Json(serde_json::json!({
                "error": "User already exists"
            })),
        ));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|_| status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({"error": "Failed to hash password"})),
        ))?;

    // Create user
    let password_hash_str = password_hash.to_string();
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        request.username,
        request.email,
        password_hash_str
    )
    .execute(&mut **db)
    .await
    .map_err(|_| status::Custom(
        Status::InternalServerError,
        Json(serde_json::json!({"error": "Failed to create user"})),
    ))?;

    let user_id = result.last_insert_rowid();

    // Create token
    let token = create_token(&user_id.to_string(), &request.email, &request.username)
        .map_err(|_| status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({"error": "Failed to create token"})),
        ))?;

    let response = status::Created::new("/users")
        .body(Json(serde_json::json!({
            "message": "User created successfully",
            "user": {
                "id": user_id,
                "username": request.username,
                "email": request.email
            },
            "token": token
        })));

    Ok(response)
}

pub async fn login(
    mut db: Connection<Db>,
    request: Json<LoginRequest>,
    cookies: &CookieJar<'_>,
) -> Result<Json<serde_json::Value>, status::Custom<Json<serde_json::Value>>> {
    // Find user
    let user = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE email = ?",
        request.email
    )
    .fetch_optional(&mut **db)
    .await
    .map_err(|_| status::Custom(
        Status::InternalServerError,
        Json(serde_json::json!({"error": "Database error"})),
    ))?;

    let user = user.ok_or_else(|| status::Custom(
        Status::Unauthorized,
        Json(serde_json::json!({"error": "Invalid credentials"})),
    ))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({"error": "Password hash error"})),
        ))?;

    if Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(status::Custom(
            Status::Unauthorized,
            Json(serde_json::json!({"error": "Invalid credentials"})),
        ));
    }

    // Create token
    let user_id = user.id.expect("User ID should be set");
    let token = create_token(&user_id.to_string(), &user.email, &user.username)
        .map_err(|_| status::Custom(
            Status::InternalServerError,
            Json(serde_json::json!({"error": "Failed to create token"})),
        ))?;

    // Set the cookie with the token
    let cookie = Cookie::build(("auth_token", token.clone()))
        .http_only(true)
        .secure(false) // Set to true in production with HTTPS
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(rocket::time::Duration::days(1));

    cookies.add(cookie);

    Ok(Json(serde_json::json!({
        "message": "Login successful",
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email
        },
        "token": token
    })))
}

pub async fn logout(cookies: &CookieJar<'_>) -> Json<serde_json::Value> {
    cookies.remove(Cookie::build(("auth_token", "")));
    Json(serde_json::json!({
        "message": "Logout successful"
    }))
}

pub async fn me(auth: crate::auth::JwtAuth) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user": {
            "id": auth.user_id,
            "email": auth.email,
            "username": auth.username
        }
    }))
}