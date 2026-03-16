use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum AppError {
    #[error("Channel not found: {0}")]
    NotFound(String),
    #[error("Telegram request failed: {0}")]
    TelegramError(String),
    #[error("Too many requests")]
    RateLimitExceeded,
    #[error("Internal server error")]
    Internal
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::TelegramError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
        };

        let body = Json(json!({"error": message, "code": status.as_u16()}));
        (status, body).into_response()
    }
}