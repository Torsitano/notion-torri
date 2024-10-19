use axum::{extract::State, http::StatusCode, Json};

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{App, ListAppsError},
};

#[tracing::instrument]
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
