use axum::{extract::State, http::StatusCode, Json};

use crate::{
    api_error::ApiError,
    apps_service::{AppsServiceTrait, KnownApp},
    backend::Backend,
};

#[utoipa::path(
    get,
    path = "/v1.0/apps/known",
    responses(
        (status = OK, description = "Successfully retrieved apps", body = Vec<KnownApp>),
        (status = UNAUTHORIZED, description = "UNAUTHORIZED", body = String),
    ),
    security(
        ("authorization" = []),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list_known_apps<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
) -> Result<(StatusCode, Json<Vec<KnownApp>>), ApiError> {
    let apps = state.apps_service.list_known_apps().await;

    Ok((StatusCode::OK, Json(apps)))
}
