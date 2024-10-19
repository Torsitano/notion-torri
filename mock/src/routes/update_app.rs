use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, AppCategory, AppState, UpdateAppError},
};

#[tracing::instrument]
pub async fn update_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    body: Json<UpdateAppHttpRequestBody>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    body.validate()?;
    let app = state.apps_service.update_app(body.0).await?;

    Ok((StatusCode::OK, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateAppHttpRequestBody {
    name: String,
    state: AppState,
    #[validate(url)]
    url: String,
    category: AppCategory,
    description: String,
    tags: Option<String>,
}

impl From<UpdateAppError> for ApiError {
    fn from(value: UpdateAppError) -> Self {
        match value {
            UpdateAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            UpdateAppError::UnexpectedError => Self::InternalServerError,
            UpdateAppError::ResourceNotFound(msg) => Self::ResourceNotFound(msg),
        }
    }
}
