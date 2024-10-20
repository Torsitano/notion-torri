use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, GetAppError},
};

#[tracing::instrument]
pub async fn get_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    Path(GetAppPathContent { id }): Path<GetAppPathContent>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    let app = state.apps_service.get_app(id).await?;

    Ok((StatusCode::OK, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct GetAppPathContent {
    pub id: u16,
}

impl From<GetAppError> for ApiError {
    fn from(value: GetAppError) -> Self {
        match value {
            GetAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            GetAppError::ResourceNotFound(id) => Self::ResourceNotFound(id.to_string()),
            GetAppError::UnexpectedError => Self::InternalServerError,
        }
    }
}
