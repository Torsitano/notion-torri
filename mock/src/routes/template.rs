use axum::{debug_handler, extract::Path, http::StatusCode, Json};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    #[schema(example = 101)]
    id: usize,
    #[schema(example = "Taylor Swift")]
    name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Group {
    #[schema(example = 101)]
    id: usize,
    #[schema(example = "Group1")]
    name: String,
    #[schema(example = json!(["Bob Jones", "Taylor Swift", "Group2"]))]
    members: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/api/user/{user_id}",
    responses(
        (status = OK, description = "Successfully retrieved user", body = User),
        (status = NOT_FOUND, description = "User not found", body = ErrorResponse),
    ),
)]
#[debug_handler]
pub async fn get_user(
    Path(user_id): Path<String>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<ErrorResponse>)> {
    match USERS.get(&user_id) {
        Some(result) => Ok((StatusCode::OK, Json(result.clone()))),
        None => {
            let response = ErrorResponse {
                message: format!("User '{user_id}' not found"),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/user",
    responses(
        (status = OK, description = "Successfully retrieved all users", body = Vec<String>),
    ),
)]
#[debug_handler]
pub async fn list_users() -> (StatusCode, Json<Vec<String>>) {
    let user_list = USERS.keys().cloned().collect();

    (StatusCode::OK, Json(user_list))
}

#[utoipa::path(
    get,
    path = "/api/group/{group_name}",
    responses(
        (status = OK, description = "Successfully retrieved group", body = Group),
        (status = NOT_FOUND, description = "Group not found", body = ErrorResponse),
    ),
)]
#[debug_handler]
pub async fn get_group(
    Path(group_name): Path<String>,
) -> Result<(StatusCode, Json<Group>), (StatusCode, Json<ErrorResponse>)> {
    match GROUPS.get(&group_name) {
        Some(result) => Ok((StatusCode::OK, Json(result.clone()))),
        None => {
            let response = Json(ErrorResponse {
                message: format!("Group '{group_name}' not found"),
            });
            Err((StatusCode::NOT_FOUND, response))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/group",
    responses(
        (status = OK, description = "Successfully retrieved list of groups", body = Vec<String>),
    ),
)]
#[debug_handler]
pub async fn list_groups() -> (StatusCode, Json<Vec<String>>) {
    let group_list = GROUPS.keys().cloned().collect();

    (StatusCode::OK, Json(group_list))
}
