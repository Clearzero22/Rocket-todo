use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::models::Claims;

pub struct JwtAuth {
    pub user_id: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug)]
pub enum JwtError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtAuth {
    type Error = JwtError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get token from Cookie or Authorization header
        let token = if let Some(cookie) = request.cookies().get("auth_token") {
            cookie.value()
        } else if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                &auth_header[7..]
            } else {
                return Outcome::Error((Status::BadRequest, JwtError::MissingToken));
            }
        } else {
            return Outcome::Forward(Status::Ok);
        };

        // Decode and validate token
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
        let validation = Validation::default();
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        );

        match decoded {
            Ok(token_data) => Outcome::Success(JwtAuth {
                user_id: token_data.claims.sub,
                email: token_data.claims.email,
                username: token_data.claims.username,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, JwtError::InvalidToken)),
        }
    }
}

pub fn create_token(user_id: &str, email: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        username: username.to_string(),
        exp: expiration,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}