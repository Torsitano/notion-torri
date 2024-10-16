use axum::{routing::get, Router};
use backend::setup;
use lambda_http::{run, Error};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app_service;
mod backend;
mod repository;
mod routes;

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let backend = setup().await;

    let router = Router::new().with_state(backend);

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    run(app).await
}
