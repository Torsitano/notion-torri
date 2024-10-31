use axum::{extract::State, http::StatusCode, Json};

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, ListAppsError},
};

#[utoipa::path(
    get,
    path = "/v1.0/apps",
    responses(
        (status = OK, description = "Successfully retrieved apps", body = Vec<App>),
        (status = UNAUTHORIZED, description = "UNAUTHORIZED", body = String),
        (status = BAD_REQUEST, description = "Bad Request", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("authorization" = []),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list_apps<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
) -> Result<(StatusCode, Json<Vec<App>>), ApiError> {
    let apps = state.apps_service.list_apps().await?;

    Ok((StatusCode::OK, Json(apps)))
}

impl From<ListAppsError> for ApiError {
    fn from(value: ListAppsError) -> Self {
        match value {
            ListAppsError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            ListAppsError::UnexpectedError => Self::InternalServerError,
        }
    }
}
