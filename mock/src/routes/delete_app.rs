use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api_error::ApiError, apps_service::AppsServiceTrait, backend::Backend,
    repository::DeleteAppError,
};

#[utoipa::path(
    delete,
    path = "/v1.0/apps/{idApp}",
    params(DeleteAppPathContent),
    responses(
        (status = OK, description = "Successfully deleted app", body = String),
        (status = BAD_REQUEST, description = "Bad Request", body = String),
        (status = UNAUTHORIZED, description = "UNAUTHORIZED", body = String),
        (status = NOT_FOUND, description = "Not found", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("authorization" = []),
    )
)]
#[tracing::instrument]
pub async fn delete_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    Path(DeleteAppPathContent { id }): Path<DeleteAppPathContent>,
) -> Result<(StatusCode, Json<String>), ApiError> {
    state.apps_service.delete_app(id).await?;

    Ok((StatusCode::OK, Json(format!("App {} deleted", id))))
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, utoipa::IntoParams)]
pub struct DeleteAppPathContent {
    pub id: u16,
}

impl From<DeleteAppError> for ApiError {
    fn from(value: DeleteAppError) -> Self {
        match value {
            DeleteAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            DeleteAppError::ResourceNotFound(id) => Self::ResourceNotFound(id.to_string()),
            DeleteAppError::UnexpectedError => Self::InternalServerError,
        }
    }
}
