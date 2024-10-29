use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use backend::setup;
use dotenv::dotenv;
use lambda_http::{run, Error};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

mod api_error;
mod apps_service;
mod auth;
mod backend;
mod repository;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::add_app,
        routes::create_app,
        routes::delete_app,
        routes::get_app,
        routes::list_apps,
        routes::list_known_apps,
        routes::search_apps,
        routes::update_app,
    ),
    modifiers(&SecurityAddon),
    security(
        ("authorization" = [
            "routes::add_app",
            "routes::create_app",
            "routes::delete_app",
            "routes::get_app",
            "routes::list_apps",
            "routes::list_known_apps",
            "routes::search_apps",
            "routes::update_app",
        ])
    ),
    components(schemas(
        routes::AddAppHttpRequestBody,
        routes::CreateAppHttpRequestBody,
        routes::UpdateAppHttpRequestBody,
        repository::models::App,
        repository::models::AppCategory,
        repository::models::AppState,       
        apps_service::KnownApp 
    ))
)]
struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("authorization"))),
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app_state = setup().await;
    let router = Router::new()
        .route("/v1.0/apps", get(routes::list_apps).post(routes::add_app))
        .route(
            "/v1.0/apps/:id",
            get(routes::get_app)
                .put(routes::update_app)
                .delete(routes::delete_app),
        )
        .route("/v1.0/apps/custom", post(routes::create_app))
        .route("/v1.0/apps/search", get(routes::search_apps))
        .route("/v1.0/apps/known", get(routes::list_known_apps))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::auth,
        ));

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state)
        .layer(tracing_layer);

    run(app).await
}
