//! Application router — all routes registered here.

use axum::{middleware, routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::auth;
use crate::controllers;
use crate::entity;
use crate::health::health_handler;
use crate::state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::product::list_products,
        controllers::product::get_product,
        controllers::product::create_product,
        controllers::product::update_product,
        controllers::product::delete_product,
    ),
    components(
        schemas(
            sakaloka_core::models::product::Product,
            controllers::product::CreateProductRequest,
            controllers::product::UpdateProductRequest,
            controllers::product::ErrorResponse,
        )
    ),
    tags(
        (name = "product", description = "Product management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

/// Builds the Axum router with all registered routes and middleware.
pub fn build_router(state: AppState) -> Router {
    let auth_routes = auth::router();

    // Entity routes require an active connection + JWT claims logic,
    // so we apply auth_middleware globally to this scope.
    let entity_routes = entity::router()
        .nest("/product", controllers::product::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth_middleware,
        ));

    Router::new()
        .merge(Router::from(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        ))
        .route("/health", get(health_handler))
        .nest("/auth", auth_routes)
        .nest("/entity", entity_routes)
        .with_state(state)
        .layer(middleware::from_fn(crate::middleware::logging_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
