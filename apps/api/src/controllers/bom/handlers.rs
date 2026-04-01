//! Handler functions for BOM endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    bom::{
        BomItemResponse, BomResponse, BomWithItemsResponse, ConsumptionResponse,
        CreateBomItemRequest, CreateBomRequest, CreateConsumptionRequest,
        CreateProductionRunRequest, ProductionRunResponse, ProductionRunWithConsumptionResponse,
        UpdateBomItemRequest, UpdateBomRequest, UpdateProductionRunRequest,
    },
    ApiResponse, PaginationParams,
};
use chrono::Utc;
use sakaloka_core::models::bom::{
    Bom, BomItem, BomStatus, Consumption, ProductionRun, ProductionRunStatus,
};

/// `GET /api/boms` — list BOMs with pagination and filters.
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<BomResponse>>>, ApiError> {
    let page = params.page() as i64;
    let per_page = params.limit() as i64;
    let org_id = claims.org_id.clone();

    let filters = sakaloka_data::postgres::bom::ListBomFilters {
        organization_id: org_id,
        product_id: None,
        status: None,
        search: params.search.clone(),
    };

    let (boms, total) = state
        .db
        .list_boms(&filters, page, per_page)
        .await
        .map_err(|e| db_err(e, "Failed to list BOMs"))?;

    let responses: Vec<BomResponse> = boms.into_iter().map(convert_bom).collect();

    Ok(Json(ApiResponse::ok(
        responses,
        &format!("Found {} BOMs", total),
    )))
}

/// `GET /api/boms/{id}` — get a BOM with its items.
pub async fn show(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<BomWithItemsResponse>>, ApiError> {
    let bom = state
        .db
        .find_bom(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;

    let items = state
        .db
        .find_bom_items(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM items"))?;

    let item_responses: Vec<BomItemResponse> = items.into_iter().map(convert_bom_item).collect();
    let bom_response = convert_bom(bom);

    Ok(Json(ApiResponse::ok(
        BomWithItemsResponse {
            bom: bom_response,
            items: item_responses,
        },
        "BOM retrieved",
    )))
}

/// `GET /api/boms/product/{product_id}` — get active BOM for a product.
pub async fn show_by_product(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(product_id): Path<String>,
) -> Result<Json<ApiResponse<BomResponse>>, ApiError> {
    let bom = state
        .db
        .find_bom_by_product(&product_id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM for product"))?
        .ok_or_else(|| ApiError::NotFound("BOM for product".to_string()))?;

    Ok(Json(ApiResponse::ok(convert_bom(bom), "BOM retrieved")))
}

/// `POST /api/boms` — create a new BOM.
pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(req): Json<CreateBomRequest>,
) -> Result<Json<ApiResponse<BomResponse>>, ApiError> {
    let org_id = claims
        .org_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("Organization ID is required".to_string()))?;

    let bom = Bom {
        id: String::new(),
        name: req.name,
        product_id: req.product_id,
        organization_id: org_id.clone(),
        version: req.version.unwrap_or_else(|| "1.0.0".to_string()),
        status: req
            .status
            .as_ref()
            .map(|s| s.parse().unwrap_or(BomStatus::Draft))
            .unwrap_or(BomStatus::Draft),
        effective_from: req.effective_from.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        effective_to: req.effective_to.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        total_cost: None,
        notes: req.notes,
        created_by: Some(format!("user:{}", claims.sub)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created = state
        .db
        .create_bom(&bom)
        .await
        .map_err(|e| db_err(e, "Failed to create BOM"))?;

    Ok(Json(ApiResponse::created(
        convert_bom(created),
        "BOM created",
    )))
}

/// `PATCH /api/boms/{id}` — update a BOM.
pub async fn update(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
    Json(req): Json<UpdateBomRequest>,
) -> Result<Json<ApiResponse<BomResponse>>, ApiError> {
    let existing = state
        .db
        .find_bom(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;

    let mut updated = existing;
    if let Some(name) = req.name {
        updated.name = name;
    }
    if let Some(version) = req.version {
        updated.version = version;
    }
    if let Some(status) = req.status {
        updated.status = status.parse().unwrap_or(BomStatus::Draft);
    }
    if let Some(notes) = req.notes {
        updated.notes = Some(notes);
    }
    if let Some(total_cost) = req.total_cost {
        updated.total_cost = Some(total_cost);
    }

    let result = state
        .db
        .update_bom(&id, &updated)
        .await
        .map_err(|e| db_err(e, "Failed to update BOM"))?;

    Ok(Json(ApiResponse::ok(convert_bom(result), "BOM updated")))
}

/// `DELETE /api/boms/{id}` — archive a BOM.
pub async fn remove(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .db
        .delete_bom(&id)
        .await
        .map_err(|e| db_err(e, "Failed to delete BOM"))?;

    Ok(Json(ApiResponse::ok((), "BOM archived")))
}

/// `POST /api/boms/{id}/activate` — activate a BOM version.
pub async fn activate(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<BomResponse>>, ApiError> {
    let existing = state
        .db
        .find_bom(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;

    let mut updated = existing;
    updated.status = BomStatus::Active;
    updated.effective_from = Some(Utc::now());

    let result = state
        .db
        .update_bom(&id, &updated)
        .await
        .map_err(|e| db_err(e, "Failed to activate BOM"))?;

    Ok(Json(ApiResponse::ok(convert_bom(result), "BOM activated")))
}

/// `GET /api/boms/{id}/items` — list items in a BOM.
pub async fn list_items(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Vec<BomItemResponse>>>, ApiError> {
    let items = state
        .db
        .find_bom_items(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM items"))?;

    let responses: Vec<BomItemResponse> = items.into_iter().map(convert_bom_item).collect();

    Ok(Json(ApiResponse::ok(responses, "BOM items retrieved")))
}

/// `POST /api/boms/{id}/items` — add an item to a BOM.
pub async fn add_item(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(bom_id): Path<String>,
    Json(req): Json<CreateBomItemRequest>,
) -> Result<Json<ApiResponse<BomItemResponse>>, ApiError> {
    let item = BomItem {
        id: String::new(),
        bom_id: bom_id.clone(),
        component_product_id: req.component_product_id,
        quantity: req.quantity,
        unit: req.unit,
        waste_percent: req.waste_percent.unwrap_or(0.0),
        yield_percent: req.yield_percent.unwrap_or(100.0),
        unit_cost: None,
        line_cost: None,
        notes: req.notes,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created = state
        .db
        .create_bom_item(&item)
        .await
        .map_err(|e| db_err(e, "Failed to create BOM item"))?;

    // Recalculate total cost
    let total = state
        .db
        .calculate_bom_cost(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to calculate cost"))?;

    // Update BOM total_cost
    let mut bom = state
        .db
        .find_bom(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;
    bom.total_cost = Some(total);
    let _ = state.db.update_bom(&bom_id, &bom).await;

    Ok(Json(ApiResponse::created(
        convert_bom_item(created),
        "BOM item added",
    )))
}

/// `PATCH /api/boms/{bom_id}/items/{item_id}` — update a BOM item.
pub async fn update_item(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path((bom_id, item_id)): Path<(String, String)>,
    Json(req): Json<UpdateBomItemRequest>,
) -> Result<Json<ApiResponse<BomItemResponse>>, ApiError> {
    let existing = state
        .db
        .find_bom_item_by_id(&item_id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM item"))?
        .ok_or_else(|| ApiError::NotFound("BOM item".to_string()))?;

    let mut updated = existing;
    if let Some(qty) = req.quantity {
        updated.quantity = qty;
    }
    if let Some(unit) = req.unit {
        updated.unit = unit;
    }
    if let Some(waste) = req.waste_percent {
        updated.waste_percent = waste;
    }
    if let Some(yield_pct) = req.yield_percent {
        updated.yield_percent = yield_pct;
    }
    if let Some(notes) = req.notes {
        updated.notes = Some(notes);
    }

    let result = state
        .db
        .update_bom_item(&item_id, &updated)
        .await
        .map_err(|e| db_err(e, "Failed to update BOM item"))?;

    // Recalculate total cost
    let total = state
        .db
        .calculate_bom_cost(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to calculate cost"))?;

    let mut bom = state
        .db
        .find_bom(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;
    bom.total_cost = Some(total);
    let _ = state.db.update_bom(&bom_id, &bom).await;

    Ok(Json(ApiResponse::ok(
        convert_bom_item(result),
        "BOM item updated",
    )))
}

/// `DELETE /api/boms/{bom_id}/items/{item_id}` — remove a BOM item.
pub async fn remove_item(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path((bom_id, item_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .db
        .delete_bom_item(&item_id)
        .await
        .map_err(|e| db_err(e, "Failed to delete BOM item"))?;

    // Recalculate total cost
    let total = state
        .db
        .calculate_bom_cost(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to calculate cost"))?;

    let mut bom = state
        .db
        .find_bom(&bom_id)
        .await
        .map_err(|e| db_err(e, "Failed to find BOM"))?
        .ok_or_else(|| ApiError::NotFound("BOM".to_string()))?;
    bom.total_cost = Some(total);
    let _ = state.db.update_bom(&bom_id, &bom).await;

    Ok(Json(ApiResponse::ok((), "BOM item removed")))
}

/// `GET /api/boms/{id}/production-runs` — list production runs.
pub async fn list_production_runs(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductionRunResponse>>>, ApiError> {
    let page = params.page() as i64;
    let per_page = params.limit() as i64;

    let (runs, _total) = state
        .db
        .list_production_runs(&id, page, per_page)
        .await
        .map_err(|e| db_err(e, "Failed to list production runs"))?;

    let responses: Vec<ProductionRunResponse> =
        runs.into_iter().map(convert_production_run).collect();

    Ok(Json(ApiResponse::ok(
        responses,
        "Production runs retrieved",
    )))
}

/// `GET /api/boms/{id}/production-runs/{run_id}` — get a production run with consumptions.
pub async fn show_production_run(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(run_id): Path<String>,
) -> Result<Json<ApiResponse<ProductionRunWithConsumptionResponse>>, ApiError> {
    let run = state
        .db
        .find_production_run(&run_id)
        .await
        .map_err(|e| db_err(e, "Failed to find production run"))?
        .ok_or_else(|| ApiError::NotFound("Production run".to_string()))?;

    let consumptions = state
        .db
        .list_consumptions(&run_id)
        .await
        .map_err(|e| db_err(e, "Failed to list consumptions"))?;

    let consumption_responses: Vec<ConsumptionResponse> =
        consumptions.into_iter().map(convert_consumption).collect();
    let run_response = convert_production_run(run);

    Ok(Json(ApiResponse::ok(
        ProductionRunWithConsumptionResponse {
            run: run_response,
            consumptions: consumption_responses,
        },
        "Production run retrieved",
    )))
}

/// `POST /api/boms/{id}/production-runs` — create a production run.
pub async fn create_production_run(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(bom_id): Path<String>,
    Json(req): Json<CreateProductionRunRequest>,
) -> Result<Json<ApiResponse<ProductionRunResponse>>, ApiError> {
    let org_id = claims
        .org_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("Organization ID is required".to_string()))?;

    let run = ProductionRun {
        id: String::new(),
        bom_id: bom_id.clone(),
        organization_id: org_id.clone(),
        quantity_produced: req.quantity_produced,
        estimated_cost: req.estimated_cost,
        actual_cost: None,
        status: req
            .status
            .as_ref()
            .map(|s| s.parse().unwrap_or(ProductionRunStatus::Planned))
            .unwrap_or(ProductionRunStatus::Planned),
        started_at: req.started_at.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        completed_at: req.completed_at.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        created_by: Some(format!("user:{}", claims.sub)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created = state
        .db
        .create_production_run(&run)
        .await
        .map_err(|e| db_err(e, "Failed to create production run"))?;

    Ok(Json(ApiResponse::created(
        convert_production_run(created),
        "Production run created",
    )))
}

/// `PATCH /api/boms/{id}/production-runs/{run_id}` — update a production run.
pub async fn update_production_run(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path((_bom_id, run_id)): Path<(String, String)>,
    Json(req): Json<UpdateProductionRunRequest>,
) -> Result<Json<ApiResponse<ProductionRunResponse>>, ApiError> {
    let existing = state
        .db
        .find_production_run(&run_id)
        .await
        .map_err(|e| db_err(e, "Failed to find production run"))?
        .ok_or_else(|| ApiError::NotFound("Production run".to_string()))?;

    let mut updated = existing;
    if let Some(qty) = req.quantity_produced {
        updated.quantity_produced = qty;
    }
    if let Some(est_cost) = req.estimated_cost {
        updated.estimated_cost = Some(est_cost);
    }
    if let Some(act_cost) = req.actual_cost {
        updated.actual_cost = Some(act_cost);
    }
    if let Some(status) = req.status {
        updated.status = status.parse().unwrap_or(ProductionRunStatus::Planned);
    }
    if let Some(started) = req.started_at {
        updated.started_at = chrono::DateTime::parse_from_rfc3339(&started)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc));
    }
    if let Some(completed) = req.completed_at {
        updated.completed_at = chrono::DateTime::parse_from_rfc3339(&completed)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc));
    }

    let result = state
        .db
        .update_production_run(&run_id, &updated)
        .await
        .map_err(|e| db_err(e, "Failed to update production run"))?;

    Ok(Json(ApiResponse::ok(
        convert_production_run(result),
        "Production run updated",
    )))
}

/// `POST /api/boms/{id}/production-runs/{run_id}/consumptions` — add a consumption record.
pub async fn add_consumption(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path((_bom_id, run_id)): Path<(String, String)>,
    Json(req): Json<CreateConsumptionRequest>,
) -> Result<Json<ApiResponse<ConsumptionResponse>>, ApiError> {
    let consumption = Consumption {
        id: String::new(),
        production_run_id: run_id,
        component_product_id: req.component_product_id,
        planned_quantity: req.planned_quantity,
        actual_quantity: req.actual_quantity,
        unit: req.unit,
        waste_quantity: req.waste_quantity.unwrap_or(0.0),
        created_at: Utc::now(),
    };

    let created = state
        .db
        .create_consumption(&consumption)
        .await
        .map_err(|e| db_err(e, "Failed to create consumption"))?;

    Ok(Json(ApiResponse::created(
        convert_consumption(created),
        "Consumption recorded",
    )))
}

/// `GET /api/boms/{id}/production-runs/{run_id}/consumptions` — list consumptions for a run.
pub async fn list_consumptions(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(run_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<ConsumptionResponse>>>, ApiError> {
    let consumptions = state
        .db
        .list_consumptions(&run_id)
        .await
        .map_err(|e| db_err(e, "Failed to list consumptions"))?;

    let responses: Vec<ConsumptionResponse> =
        consumptions.into_iter().map(convert_consumption).collect();

    Ok(Json(ApiResponse::ok(responses, "Consumptions retrieved")))
}

// Helper functions to convert domain models to DTOs
fn convert_bom(bom: Bom) -> BomResponse {
    BomResponse {
        id: bom.id,
        name: bom.name,
        product_id: bom.product_id,
        organization_id: bom.organization_id,
        version: bom.version,
        status: bom.status.to_string(),
        effective_from: bom
            .effective_from
            .map(|dt: chrono::DateTime<chrono::Utc>| dt.to_rfc3339()),
        effective_to: bom
            .effective_to
            .map(|dt: chrono::DateTime<chrono::Utc>| dt.to_rfc3339()),
        total_cost: bom.total_cost,
        notes: bom.notes,
        created_by: bom.created_by,
        created_at: bom.created_at.to_rfc3339(),
        updated_at: bom.updated_at.to_rfc3339(),
    }
}

fn convert_bom_item(item: BomItem) -> BomItemResponse {
    BomItemResponse {
        id: item.id,
        bom_id: item.bom_id,
        component_product_id: item.component_product_id,
        quantity: item.quantity,
        unit: item.unit,
        waste_percent: item.waste_percent,
        yield_percent: item.yield_percent,
        unit_cost: item.unit_cost,
        line_cost: item.line_cost,
        notes: item.notes,
        created_at: item.created_at.to_rfc3339(),
        updated_at: item.updated_at.to_rfc3339(),
    }
}

fn convert_production_run(run: ProductionRun) -> ProductionRunResponse {
    ProductionRunResponse {
        id: run.id,
        bom_id: run.bom_id,
        organization_id: run.organization_id,
        quantity_produced: run.quantity_produced,
        estimated_cost: run.estimated_cost,
        actual_cost: run.actual_cost,
        status: run.status.to_string(),
        started_at: run
            .started_at
            .map(|dt: chrono::DateTime<chrono::Utc>| dt.to_rfc3339()),
        completed_at: run
            .completed_at
            .map(|dt: chrono::DateTime<chrono::Utc>| dt.to_rfc3339()),
        created_by: run.created_by,
        created_at: run.created_at.to_rfc3339(),
        updated_at: run.updated_at.to_rfc3339(),
    }
}

fn convert_consumption(c: Consumption) -> ConsumptionResponse {
    ConsumptionResponse {
        id: c.id,
        production_run_id: c.production_run_id,
        component_product_id: c.component_product_id,
        planned_quantity: c.planned_quantity,
        actual_quantity: c.actual_quantity,
        unit: c.unit,
        waste_quantity: c.waste_quantity,
        created_at: c.created_at.to_rfc3339(),
    }
}
