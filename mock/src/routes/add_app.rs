use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api_error::ApiError,
    apps_service::AppsServiceTrait,
    backend::Backend,
    repository::{AddAppError, AddAppRequest, App},
};

#[tracing::instrument]
pub async fn add_app<AS: AppsServiceTrait>(
    State(state): State<Backend<AS>>,
    body: Json<AddAppHttpRequestBody>,
) -> Result<(StatusCode, Json<App>), ApiError> {
    let app = state.apps_service.add_app(body.0).await?;

    Ok((StatusCode::CREATED, Json(app)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAppHttpRequestBody {
    #[serde(rename = "idApp")]
    pub id_app: u16,
}

impl From<AddAppError> for ApiError {
    fn from(value: AddAppError) -> Self {
        match value {
            AddAppError::ValidationError(msg) => Self::ValidationError(msg.to_string()),
            AddAppError::ResourceNotFound(id) => Self::ResourceNotFound(id.to_string()),
            AddAppError::ResourceAlreadyExists { name } => Self::ResourceAlreadyExists(name),
            AddAppError::UnexpectedError => Self::InternalServerError,
        }
    }
}

impl From<AddAppHttpRequestBody> for AddAppRequest {
    fn from(value: AddAppHttpRequestBody) -> Self {
        Self {
            id_app: value.id_app,
        }
    }
}
