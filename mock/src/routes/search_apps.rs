use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api_error::ApiError, apps_service::AppsServiceTrait, backend::Backend, repository::App,
};

#[utoipa::path(
    get,
    path = "/v1.0/apps/search",
    params(SearchAppsQueryParams),
    responses(
        (status = OK, description = "List of apps matching query", body = Vec<App>),
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
pub async fn search_apps<AS: AppsServiceTrait>(
    Query(params): Query<SearchAppsQueryParams>,
    State(state): State<Backend<AS>>,
) -> Result<(StatusCode, Json<Vec<App>>), ApiError> {
    let apps = state.apps_service.search_apps(params).await?;

    Ok((StatusCode::OK, Json(apps)))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::IntoParams)]
pub struct SearchAppsQueryParams {
    pub query: String,
}
