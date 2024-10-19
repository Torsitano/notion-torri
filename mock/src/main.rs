use axum::{
    routing::{get, post},
    Router,
};
use backend::setup;
use dotenv::dotenv;
use lambda_http::{run, Error};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_error;
mod apps_service;
mod backend;
mod repository;
mod routes;

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
        |request: &axum::extract::Request<axum::body::Body>| {
            let uri = request.uri().to_string();
            tracing::info_span!("http_request", method = ?request.method(), uri)
        },
    );

    let app_state = setup().await;

    let router = Router::new();

    let app = router
        .route("/v1.0/apps", get(routes::list_apps).post(routes::add_app))
        // .route(
        //     "/v1.0/apps/:id",
        //     get(routes::get_app).put(routes::update_app),
        // )
        .route("/v1.0/apps/custom", post(routes::create_app))
        // .route("/v1.0/apps/search", get(routes::search_apps))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(trace_layer)
        .with_state(app_state);

    run(app).await
}
