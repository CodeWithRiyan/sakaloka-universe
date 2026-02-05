#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

use crate::models::{
    _entities::{
        organizations::{ActiveModel, Entity, Model},
        sea_orm_active_enums::OrganizationType,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub code: Option<String>,
    pub r#type: Option<OrganizationType>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        if let Some(ref name) = self.name {
            item.name = Set(name.clone());
        }
        if let Some(ref code) = self.code {
            item.code = Set(Some(code.clone()));
        }
        if let Some(ref r#type) = self.r#type {
            item.r#type = Set(r#type.clone());
        }
        if let Some(ref address) = self.address {
            item.address = Set(Some(address.clone()));
        }
        if let Some(ref phone) = self.phone {
            item.phone = Set(Some(phone.clone()));
        }
        item.updated_at = Set(Utc::now().naive_utc());
    }
}

async fn load_item(ctx: &AppContext, id: &str) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        is_active: Set(true),
        timezone: Set("UTC".to_string()), // Default
        currency: Set("IDR".to_string()), // Default
        settings: Set(json!({})),
        r#type: Set(OrganizationType::Company), // Default if not provided?
        ..Default::default()
    };
    
    // Override defaults with params if provided
    params.update(&mut item);

    // Validate name is set (since it's Option in Params but required in DB)
    // Validate name is set
    // Correction: 'self' is not available here. checking params directly.
    if params.name.is_none() {
         return bad_request("Name is required");
    }

    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, &id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, &id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, &id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/organizations") // Removed trailing slash for consistency
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}
