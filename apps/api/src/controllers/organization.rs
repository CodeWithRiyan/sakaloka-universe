//! Organization management controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    organization::{CreateOrgRequest, OrganizationResponse, SelectOrgRequest, UpdateOrgRequest},
    record_id_to_string, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// Registers all `/organizations` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/organizations", get(list))
        .route("/organizations/current", get(current))
        .route("/organizations/{id}", get(show))
        .route_layer(RequireScope::new(Scope::EntityRead));

    let write_routes = Router::new()
        .route("/organizations", post(create))
        .route("/organizations/select", post(select))
        .route(
            "/organizations/{id}",
            axum::routing::patch(update).delete(remove),
        )
        .route_layer(RequireScope::new(Scope::EntityWrite));

    Router::new().merge(read_routes).merge(write_routes)
}

/// `GET /api/organizations` — list organizations the caller has access to.
async fn list(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrganizationResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_organizations(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count organizations");
            ApiError::Internal(anyhow::anyhow!("Failed to count organizations"))
        })?;

    let items = state
        .db
        .list_organizations(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list organizations");
            ApiError::Internal(anyhow::anyhow!("Failed to list organizations"))
        })?;

    let responses: Vec<OrganizationResponse> =
        items.iter().map(OrganizationResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Organizations retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/organizations/current` — fetch the caller's current organization.
async fn current(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let user = state
        .db
        .find_user_by_id(&claims.sub)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find user")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("User not found")))?;

    let org_id = user
        .organization_id
        .as_ref()
        .map(record_id_to_string)
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))?;

    let org = state.db.find_organization(&org_id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find organization");
        ApiError::Internal(anyhow::anyhow!("Failed to find organization"))
    })?;

    match org {
        Some(o) => Ok(Json(ApiResponse::ok(
            OrganizationResponse::from_model(&o),
            "Current organization retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Organization"))),
    }
}

/// `POST /api/organizations/select` — switch the caller's active organization.
async fn select(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<SelectOrgRequest>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    // Verify the target organization exists and is active
    let org = state
        .db
        .find_organization(&payload.organization_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find organization")))?;

    let org = match org {
        Some(o) if o.is_active => o,
        Some(_) => {
            return Ok(Json(ApiResponse::error(
                "org_inactive",
                "The selected organization is not active",
            )))
        }
        None => return Ok(Json(ApiResponse::not_found("Organization"))),
    };

    // Update the user's organization_id
    state
        .db
        .update_user_organization(&claims.sub, &payload.organization_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to switch organization");
            ApiError::Internal(anyhow::anyhow!("Failed to switch organization"))
        })?;

    Ok(Json(ApiResponse::ok(
        OrganizationResponse::from_model(&org),
        "Organization switched",
    )))
}

/// `GET /api/organizations/:id` — fetch a single organization.
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let org = state.db.find_organization(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find organization");
        ApiError::Internal(anyhow::anyhow!("Failed to find organization"))
    })?;

    match org {
        Some(o) => Ok(Json(ApiResponse::ok(
            OrganizationResponse::from_model(&o),
            "Organization retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Organization"))),
    }
}

/// `POST /api/organizations` — create a new organization.
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }

    let result = state
        .db
        .create_organization(&payload.name, &payload.org_type, Some(&claims.sub))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create organization");
            ApiError::Internal(anyhow::anyhow!("Failed to create organization"))
        })?;

    // Apply additional fields via update if present
    let org_id = record_id_to_string(&result.id);
    let mut updates = serde_json::Map::new();
    if let Some(code) = payload.code {
        updates.insert("code".to_string(), serde_json::Value::String(code));
    }
    if let Some(description) = payload.description {
        updates.insert(
            "description".to_string(),
            serde_json::Value::String(description),
        );
    }
    if let Some(parent_id) = payload.parent_id {
        updates.insert(
            "parent_id".to_string(),
            serde_json::Value::String(parent_id),
        );
    }
    if let Some(email) = payload.email {
        updates.insert("email".to_string(), serde_json::Value::String(email));
    }
    if let Some(phone) = payload.phone {
        updates.insert("phone".to_string(), serde_json::Value::String(phone));
    }
    if let Some(website) = payload.website {
        updates.insert("website".to_string(), serde_json::Value::String(website));
    }
    if let Some(address) = payload.address {
        updates.insert("address".to_string(), serde_json::Value::String(address));
    }
    if let Some(city) = payload.city {
        updates.insert("city".to_string(), serde_json::Value::String(city));
    }
    if let Some(st) = payload.state {
        updates.insert("state".to_string(), serde_json::Value::String(st));
    }
    if let Some(country) = payload.country {
        updates.insert("country".to_string(), serde_json::Value::String(country));
    }
    if let Some(postal_code) = payload.postal_code {
        updates.insert(
            "postal_code".to_string(),
            serde_json::Value::String(postal_code),
        );
    }
    if let Some(tax_number) = payload.tax_number {
        updates.insert(
            "tax_number".to_string(),
            serde_json::Value::String(tax_number),
        );
    }
    if let Some(registration_number) = payload.registration_number {
        updates.insert(
            "registration_number".to_string(),
            serde_json::Value::String(registration_number),
        );
    }
    if let Some(logo) = payload.logo {
        updates.insert("logo".to_string(), serde_json::Value::String(logo));
    }
    if let Some(settings) = payload.settings {
        updates.insert("settings".to_string(), settings);
    }

    let final_org = if updates.is_empty() {
        result
    } else {
        state
            .db
            .update_organization(&org_id, &serde_json::Value::Object(updates))
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update organization extras");
                ApiError::Internal(anyhow::anyhow!("Failed to update organization"))
            })?
    };

    Ok(Json(ApiResponse::created(
        OrganizationResponse::from_model(&final_org),
        "Organization created",
    )))
}

/// `PATCH /api/organizations/:id` — update an existing organization.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOrgRequest>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let existing = state.db.find_organization(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find organization for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find organization"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Organization")));
    }

    let mut updates = serde_json::Map::new();
    if let Some(name) = payload.name {
        updates.insert("name".to_string(), serde_json::Value::String(name));
    }
    if let Some(org_type) = payload.org_type {
        updates.insert("type".to_string(), serde_json::Value::String(org_type));
    }
    if let Some(code) = payload.code {
        updates.insert("code".to_string(), serde_json::Value::String(code));
    }
    if let Some(description) = payload.description {
        updates.insert(
            "description".to_string(),
            serde_json::Value::String(description),
        );
    }
    if let Some(email) = payload.email {
        updates.insert("email".to_string(), serde_json::Value::String(email));
    }
    if let Some(phone) = payload.phone {
        updates.insert("phone".to_string(), serde_json::Value::String(phone));
    }
    if let Some(website) = payload.website {
        updates.insert("website".to_string(), serde_json::Value::String(website));
    }
    if let Some(address) = payload.address {
        updates.insert("address".to_string(), serde_json::Value::String(address));
    }
    if let Some(city) = payload.city {
        updates.insert("city".to_string(), serde_json::Value::String(city));
    }
    if let Some(st) = payload.state {
        updates.insert("state".to_string(), serde_json::Value::String(st));
    }
    if let Some(country) = payload.country {
        updates.insert("country".to_string(), serde_json::Value::String(country));
    }
    if let Some(postal_code) = payload.postal_code {
        updates.insert(
            "postal_code".to_string(),
            serde_json::Value::String(postal_code),
        );
    }
    if let Some(tax_number) = payload.tax_number {
        updates.insert(
            "tax_number".to_string(),
            serde_json::Value::String(tax_number),
        );
    }
    if let Some(registration_number) = payload.registration_number {
        updates.insert(
            "registration_number".to_string(),
            serde_json::Value::String(registration_number),
        );
    }
    if let Some(logo) = payload.logo {
        updates.insert("logo".to_string(), serde_json::Value::String(logo));
    }
    if let Some(settings) = payload.settings {
        updates.insert("settings".to_string(), settings);
    }
    if let Some(is_active) = payload.is_active {
        updates.insert("is_active".to_string(), serde_json::Value::Bool(is_active));
    }

    let updated = state
        .db
        .update_organization(&id, &serde_json::Value::Object(updates))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update organization");
            ApiError::Internal(anyhow::anyhow!("Failed to update organization"))
        })?;

    Ok(Json(ApiResponse::ok(
        OrganizationResponse::from_model(&updated),
        "Organization updated",
    )))
}

/// `DELETE /api/organizations/:id` — deactivate an organization.
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state.db.find_organization(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find organization for deactivation");
        ApiError::Internal(anyhow::anyhow!("Failed to find organization"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Organization")));
    }

    // Soft-deactivate
    let updates = serde_json::json!({ "is_active": false });
    state
        .db
        .update_organization(&id, &updates)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to deactivate organization");
            ApiError::Internal(anyhow::anyhow!("Failed to deactivate organization"))
        })?;

    Ok(Json(ApiResponse::ok((), "Organization deactivated")))
}
