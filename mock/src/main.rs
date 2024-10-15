use axum::{routing::get, Router};
use lambda_http::{run, Error};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

    let router = Router::new();

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    run(app).await
}
