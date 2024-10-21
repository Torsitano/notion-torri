use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tracing::warn;

pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if !(auth_header != "test") {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        warn!("No authorization header provided");
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}
