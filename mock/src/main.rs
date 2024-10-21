use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use backend::setup;
use dotenv::dotenv;
use lambda_http::{run, Error};
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

    let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
        |request: &axum::extract::Request<axum::body::Body>| {
            let uri = request.uri().to_string();
            tracing::info_span!("http_request", method = ?request.method(), uri)
        },
    );

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
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::auth,
        ));

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state)
        .layer(trace_layer);

    run(app).await
}
