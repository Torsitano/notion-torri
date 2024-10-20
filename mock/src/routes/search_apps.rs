use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api_error::ApiError, apps_service::AppsServiceTrait, backend::Backend, repository::App,
};

#[tracing::instrument]
pub async fn search_apps<AS: AppsServiceTrait>(
    Query(params): Query<SearchAppsQueryParams>,
    State(state): State<Backend<AS>>,
) -> Result<(StatusCode, Json<Vec<App>>), ApiError> {
    let apps = state.apps_service.search_apps(params).await?;

    Ok((StatusCode::OK, Json(apps)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAppsQueryParams {
    pub query: String,
}

// impl From<SearchAppsError> for ApiError {
//     fn from(value: SearchAppsError) -> Self {
//         match value {
//             SearchAppsError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
//             SearchAppsError::UnexpectedError => Self::InternalServerError,
//         }
//     }
// }
