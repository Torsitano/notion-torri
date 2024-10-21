use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, AppCategory, AppState, UpdateAppError},
};

#[utoipa::path(
    put,
    path = "/v1.0/apps",
    params(UpdateAppPathContent),
    request_body = UpdateAppHttpRequestBody,
    responses(
        (status = OK, description = "Successfully updated app", body = App),
        (status = BAD_REQUEST, description = "Bad Request", body = String),
        (status = UNAUTHORIZED, description = "UNAUTHORIZED", body = String),
        (status = NOT_FOUND, description = "Not found", body = String),
        (status = CONFLICT, description = "App already exists", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("authorization" = []),
    )
)]
#[tracing::instrument]
pub async fn update_app<AS: AppsServiceTrait>(
    Path(UpdateAppPathContent { id }): Path<UpdateAppPathContent>,
    State(state): State<Backend<AS>>,
    body: Json<UpdateAppHttpRequestBody>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    body.validate()?;
    let app = state.apps_service.update_app(body.0, id).await?;

    Ok((StatusCode::OK, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, utoipa::IntoParams)]
pub struct UpdateAppPathContent {
    pub id: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateAppHttpRequestBody {
    pub name: Option<String>,
    pub state: Option<AppState>,
    #[validate(url)]
    pub url: Option<String>,
    pub category: Option<AppCategory>,
    pub description: Option<String>,
    pub tags: Option<String>,
}

impl From<UpdateAppError> for ApiError {
    fn from(value: UpdateAppError) -> Self {
        match value {
            UpdateAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            UpdateAppError::UnexpectedError => Self::InternalServerError,
            UpdateAppError::ResourceNotFound(msg) => Self::ResourceNotFound(msg.to_string()),
        }
    }
}
