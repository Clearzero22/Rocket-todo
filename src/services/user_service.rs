//! User Service Implementation

use crate::services::{error::{ServiceError, ServiceResult}, DbConnection};
use crate::models::{CreateUserRequest, LoginRequest, User, UserResponse};
use crate::auth::jwt::create_token;

/// User service handles user-related business logic
#[derive(Debug)]
pub struct UserService;

#[derive(Debug)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new() -> Self {
        Self
    }

    /// Create a new user
    pub async fn create(
        &self,
        mut db: DbConnection,
        request: CreateUserRequest,
    ) -> ServiceResult<UserResponse> {
        // Validate request
        if request.username.trim().is_empty() {
            return Err(ServiceError::validation("Username cannot be empty"));
        }

        if request.email.trim().is_empty() {
            return Err(ServiceError::validation("Email cannot be empty"));
        }

        if request.password.trim().is_empty() {
            return Err(ServiceError::validation("Password cannot be empty"));
        }

        // Check if user already exists
        let existing = sqlx::query!(
            "SELECT id FROM users WHERE username = ? OR email = ?",
            request.username, request.email
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        if existing.is_some() {
            return Err(ServiceError::conflict("Username or email already exists"));
        }

        // Hash password
        let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)
            .map_err(|e| ServiceError::internal(&format!("Password hashing failed: {}", e)))?;

        // Insert user
        let result = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
            request.username, request.email, password_hash.to_string()
        )
        .execute(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        let id = result.last_insert_rowid();

        // Get the created record
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await
        .map_err(ServiceError::Database)?;

        Ok(UserResponse::from(user))
    }

    /// Authenticate user and return token
    pub async fn authenticate(
        &self,
        mut db: DbConnection,
        request: LoginRequest,
    ) -> ServiceResult<AuthResponse> {
        // Get user by email
        let user_data = sqlx::query!(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = ?",
            request.email
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::unauthorized("Invalid email or password"))?;

        // Verify password
        let parsed_hash = argon2::PasswordHash::new(&user_data.password_hash)
            .map_err(|e| ServiceError::internal(&format!("Password hash parsing failed: {}", e)))?;

        let argon2 = argon2::Argon2::default();
        argon2
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| ServiceError::unauthorized("Invalid email or password"))?;

        let user = User {
            id: user_data.id,
            username: user_data.username,
            email: user_data.email,
            password_hash: user_data.password_hash,
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
        };

        let user_response = UserResponse::from(user.clone());

        // Create JWT token
        let token = create_token(&user)
            .map_err(|e| ServiceError::internal(&format!("Token creation failed: {}", e)))?;

        Ok(AuthResponse {
            user: user_response,
            token,
        })
    }

    /// Get a user by ID
    pub async fn get_by_id(&self, mut db: DbConnection, id: i64) -> ServiceResult<UserResponse> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("User with id {}", id)))?;

        Ok(UserResponse::from(user))
    }

    /// Get user by email
    pub async fn get_by_email(&self, mut db: DbConnection, email: &str) -> ServiceResult<UserResponse> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("User with email {}", email)))?;

        Ok(UserResponse::from(user))
    }

    /// Get user with password hash (for authentication)
    pub async fn get_with_password(&self, mut db: DbConnection, email: &str) -> ServiceResult<(User, String)> {
        let user_data = sqlx::query!(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&mut **db)
        .await
        .map_err(ServiceError::Database)?
        .ok_or_else(|| ServiceError::not_found(&format!("User with email {}", email)))?;

        let user = User {
            id: user_data.id,
            username: user_data.username,
            email: user_data.email,
            password_hash: user_data.password_hash.clone(),
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
        };

        Ok((user, user_data.password_hash))
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

// Add From implementation for UserResponse
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.unwrap_or(0),
            username: user.username,
            email: user.email,
            created_at: chrono::DateTime::from_naive_utc_and_offset(
                user.created_at.unwrap_or(chrono::NaiveDateTime::default()),
                chrono::Utc
            ),
        }
    }
}