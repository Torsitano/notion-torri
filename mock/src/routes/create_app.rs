use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, AppCategory, AppState, CreateAppError},
};

#[tracing::instrument]
pub async fn create_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    body: Json<CreateAppHttpRequestBody>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    body.validate()?;
    let app = state.apps_service.create_app(body.0).await?;

    Ok((StatusCode::CREATED, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAppHttpRequestBody {
    pub name: String,
    pub state: AppState,
    #[validate(url)]
    pub url: String,
    pub category: AppCategory,
    pub description: String,
    pub tags: Option<String>,
}

impl From<CreateAppError> for ApiError {
    fn from(value: CreateAppError) -> Self {
        match value {
            CreateAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            CreateAppError::ResourceAlreadyExists { name } => Self::ResourceAlreadyExists(name),
            CreateAppError::UnexpectedError => Self::InternalServerError,
        }
    }
}
