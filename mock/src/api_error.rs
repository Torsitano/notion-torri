use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use validator::{ValidationError, ValidationErrors};

#[derive(thiserror::Error, Debug, utoipa::ToSchema)]
pub enum ApiError {
    #[error("Not Found: {0}")]
    ResourceNotFound(String),

    #[error("Already exists: {0}")]
    ResourceAlreadyExists(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unexpected error occurred")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            ApiError::ResourceNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::ResourceAlreadyExists(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                &"An unexpected error has occurred".to_string(),
            ),
        };

        let body = json!({
            "error": error_message
        });

        (status, Json(body)).into_response()
    }
}

impl From<ValidationError> for ApiError {
    fn from(value: ValidationError) -> Self {
        let message = if let Some(cow) = value.message {
            cow.into_owned()
        } else {
            "Validation Error".to_string()
        };

        ApiError::ValidationError(message)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        let message = format!("{:#?}", value);

        ApiError::ValidationError(message)
    }
}
