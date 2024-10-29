use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::{debug, warn};

use crate::{apps_service::AppsServiceTrait, backend::Backend};

pub async fn auth<AS>(
    State(state): State<Backend<AS>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    AS: AppsServiceTrait,
{
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    debug!("Auth header: {:#?}", auth_header);

    if let Some(auth_header) = auth_header {
        if auth_header != format!("Bearer {}", state.auth_api_key) {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        warn!("No authorization header provided");
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}
