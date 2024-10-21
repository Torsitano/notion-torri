use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, AppCategory, AppState, CreateAppError},
};

#[utoipa::path(
    post,
    path = "/v1.0/apps/custom",
    request_body = CreateAppHttpRequestBody,
    responses(
        (status = OK, description = "Successfully created app", body = App),
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
#[tracing::instrument(skip(state))]
pub async fn create_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    body: Json<CreateAppHttpRequestBody>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    body.validate()?;
    let app = state.apps_service.create_app(body.0).await?;

    Ok((StatusCode::CREATED, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
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
