use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
#[allow(unused_imports)]
use sakaloka_core::models::product::Product;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload for creating a product.
#[derive(Deserialize, ToSchema)]
pub struct CreateProductRequest {
    /// Product name.
    pub name: String,
    /// Unique SKU.
    pub sku: String,
    /// Price in cents.
    pub price: u64,
    /// Optional description.
    pub description: Option<String>,
}

/// Request payload for updating a product.
#[derive(Deserialize, ToSchema)]
pub struct UpdateProductRequest {
    /// Updated name.
    pub name: Option<String>,
    /// Updated price in cents.
    pub price: Option<u64>,
    /// Updated description.
    pub description: Option<String>,
}

/// Standard error response following RFC 9457.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code.
    pub error: String,
    /// Human-readable message.
    pub message: String,
}

/// Mounts the product CRUD routes.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products).post(create_product))
        .route(
            "/{id}",
            get(get_product).put(update_product).delete(delete_product),
        )
}

#[utoipa::path(
    get,
    path = "/entity/product",
    tag = "product",
    responses(
        (status = 200, description = "List all products", body = Vec<Product>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
async fn list_products(State(state): State<AppState>) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match db.list_products().await {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/entity/product/{id}",
    tag = "product",
    params(
        ("id" = String, Path, description = "Product ID (e.g. product:xxx)")
    ),
    responses(
        (status = 200, description = "Product found", body = Product),
        (status = 404, description = "Product not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
async fn get_product(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match db.find_product(&id).await {
        Ok(Some(product)) => (StatusCode::OK, Json(product)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".into(),
                message: "Product not found".into(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/entity/product",
    tag = "product",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created successfully", body = Product),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match db
        .create_product(
            &payload.name,
            &payload.sku,
            payload.price,
            payload.description.as_deref(),
        )
        .await
    {
        Ok(product) => (StatusCode::CREATED, Json(product)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/entity/product/{id}",
    tag = "product",
    params(
        ("id" = String, Path, description = "Product ID (e.g. product:xxx)")
    ),
    request_body = UpdateProductRequest,
    responses(
        (status = 200, description = "Product updated successfully", body = Product),
        (status = 404, description = "Product not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductRequest>,
) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match db
        .update_product(
            &id,
            payload.name.as_deref(),
            payload.price,
            payload.description.as_deref(),
        )
        .await
    {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/entity/product/{id}",
    tag = "product",
    params(
        ("id" = String, Path, description = "Product ID (e.g. product:xxx)")
    ),
    responses(
        (status = 204, description = "Product deleted successfully"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match db.delete_product(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}
