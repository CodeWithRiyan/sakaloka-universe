use loco_rs::prelude::*;
use chrono::NaiveDateTime;
use crate::models::{
    _entities::users,
    sync::Sync as SyncLogic,
};
use crate::controllers::sync_dtos::PushPayloadDto; // Import DTOs

async fn load_user(ctx: &AppContext, auth: &auth::JWT) -> Result<users::Model> {
    users::Model::find_by_claims_key(&ctx.db, &auth.claims.pid).await
        .map_err(|_| Error::Unauthorized("Unauthorized".into()))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/api/sync/bootstrap",
    responses(
        (status = 200, description = "Bootstrap sync data", body = String)
    )
)]
pub async fn bootstrap(
    auth: auth::JWT,
    State(ctx): State<AppContext>
) -> Result<Response> {
    let user = load_user(&ctx, &auth).await?;
    let org_id = user.selected_organization_id.ok_or_else(|| Error::BadRequest("No organization selected".into()))?;
    
    // Bootstrap: fetch all (since epoch)
    let epoch = NaiveDateTime::parse_from_str("1970-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let data = SyncLogic::incremental(&ctx.db, &org_id, epoch).await?;
    
    format::json(data)
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/api/sync/incremental",
    params(
        ("since" = String, Query, description = "Timestamp from when to fetch updates (ISO 8601)")
    ),
    responses(
        (status = 200, description = "Incremental sync data", body = String)
    )
)]
pub async fn incremental(
    auth: auth::JWT,
    Query(params): Query<serde_json::Value>, // Using serde_json::Value to extract 'since' string manually or map struct
    State(ctx): State<AppContext>
) -> Result<Response> {
    let user = load_user(&ctx, &auth).await?;
    let org_id = user.selected_organization_id.ok_or_else(|| Error::BadRequest("No organization selected".into()))?;

    // Extract 'since' from query param
    let since_str = params.get("since").and_then(|v| v.as_str()).ok_or_else(|| Error::BadRequest("Missing 'since' parameter".into()))?;
    
    let since = NaiveDateTime::parse_from_str(since_str, "%Y-%m-%dT%H:%M:%S%.fZ").ok()
        .or_else(|| NaiveDateTime::parse_from_str(since_str, "%Y-%m-%dT%H:%M:%S").ok())
        .ok_or_else(|| Error::BadRequest("Invalid 'since' format".into()))?;

    let data = SyncLogic::incremental(&ctx.db, &org_id, since).await?;
    format::json(data)
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/api/sync/push",
    request_body = PushPayloadDto,
    responses(
        (status = 200, description = "Push result (ID mapping)", body = String)
    )
)]
pub async fn push(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(payload): Json<PushPayloadDto>
) -> Result<Response> {
    let user = load_user(&ctx, &auth).await?;
    let org_id = user.selected_organization_id.ok_or_else(|| Error::BadRequest("No organization selected".into()))?;
    
    let result = SyncLogic::push(&ctx.db, &org_id, &user.id, payload).await?;
    format::json(result)
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/sync")
        .add("/bootstrap", get(bootstrap))
        .add("/incremental", get(incremental)) // Query params handled in handler
        .add("/push", post(push))
}
