//! Handler functions for organization endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::helpers::patch_builder::PatchBuilder;
use crate::views::{
    organization::{CreateOrgRequest, OrganizationResponse, SelectOrgRequest, UpdateOrgRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// `GET /api/organizations` — list organizations the caller has access to.
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrganizationResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;
    let org_id = claims.org_id.as_deref();

    let search = params.search.as_deref();
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();

    let (count_result, list_result) = tokio::join!(
        state.db.count_organizations(org_id, search),
        state
            .db
            .list_organizations(org_id, limit, start, search, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count organizations"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list organizations"))?;

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
pub async fn current(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

    let org = state
        .db
        .find_organization(&org_id)
        .await
        .map_err(|e| db_err(e, "Failed to find organization"))?;

    match org {
        Some(o) => Ok(Json(ApiResponse::ok(
            OrganizationResponse::from_model(&o),
            "Current organization retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Organization"))),
    }
}

/// `POST /api/organizations/select` — switch the caller's active organization.
pub async fn select(
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
        .map_err(|e| db_err(e, "Failed to switch organization"))?;

    Ok(Json(ApiResponse::ok(
        OrganizationResponse::from_model(&org),
        "Organization switched",
    )))
}

/// `GET /api/organizations/:id` — fetch a single organization.
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let org = state
        .db
        .find_organization(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find organization"))?;

    match org {
        Some(o) => Ok(Json(ApiResponse::ok(
            OrganizationResponse::from_model(&o),
            "Organization retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Organization"))),
    }
}

/// `POST /api/organizations` — create a new organization.
pub async fn create(
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
        .map_err(|e| db_err(e, "Failed to create organization"))?;

    // Apply additional fields via update if present
    let org_id = result.id.clone();
    let extras = PatchBuilder::new()
        .set_string("code", payload.code)
        .set_string("description", payload.description)
        .set_string("parent_id", payload.parent_id)
        .set_string("email", payload.email)
        .set_string("phone", payload.phone)
        .set_string("website", payload.website)
        .set_string("address", payload.address)
        .set_string("city", payload.city)
        .set_string("state", payload.state)
        .set_string("country", payload.country)
        .set_string("postal_code", payload.postal_code)
        .set_string("tax_number", payload.tax_number)
        .set_string("registration_number", payload.registration_number)
        .set_string("logo", payload.logo)
        .set_value("settings", payload.settings);

    let final_org = if extras.is_empty() {
        result
    } else {
        state
            .db
            .update_organization(&org_id, &extras.build())
            .await
            .map_err(|e| db_err(e, "Failed to update organization extras"))?
    };

    Ok(Json(ApiResponse::created(
        OrganizationResponse::from_model(&final_org),
        "Organization created",
    )))
}

/// `PATCH /api/organizations/:id` — update an existing organization.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOrgRequest>,
) -> Result<Json<ApiResponse<OrganizationResponse>>, ApiError> {
    let existing = state
        .db
        .find_organization(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find organization"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Organization")));
    }

    let updates = PatchBuilder::new()
        .set_string("name", payload.name)
        .set_string("type", payload.org_type)
        .set_string("code", payload.code)
        .set_string("description", payload.description)
        .set_string("email", payload.email)
        .set_string("phone", payload.phone)
        .set_string("website", payload.website)
        .set_string("address", payload.address)
        .set_string("city", payload.city)
        .set_string("state", payload.state)
        .set_string("country", payload.country)
        .set_string("postal_code", payload.postal_code)
        .set_string("tax_number", payload.tax_number)
        .set_string("registration_number", payload.registration_number)
        .set_string("logo", payload.logo)
        .set_value("settings", payload.settings)
        .set_bool("is_active", payload.is_active)
        .build();

    let updated = state
        .db
        .update_organization(&id, &updates)
        .await
        .map_err(|e| db_err(e, "Failed to update organization"))?;

    Ok(Json(ApiResponse::ok(
        OrganizationResponse::from_model(&updated),
        "Organization updated",
    )))
}

/// `DELETE /api/organizations/:id` — deactivate an organization.
pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state
        .db
        .find_organization(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find organization"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Organization")));
    }

    // Soft-deactivate
    let updates = serde_json::json!({ "is_active": false });
    state
        .db
        .update_organization(&id, &updates)
        .await
        .map_err(|e| db_err(e, "Failed to deactivate organization"))?;

    Ok(Json(ApiResponse::ok((), "Organization deactivated")))
}
