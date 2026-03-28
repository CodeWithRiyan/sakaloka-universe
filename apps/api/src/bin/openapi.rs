//! Generates the OpenAPI JSON specification for the Sakaloka API.
//!
//! Run with: `cargo run -p sakaloka-api --bin openapi`
//! Pipe to a file: `cargo run -p sakaloka-api --bin openapi > openapi.json`

use utoipa::OpenApi;

/// The root OpenAPI document aggregating all schemas and paths.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sakaloka API",
        version = "0.1.0",
        description = "Sakaloka-Universe Earth REST API"
    ),
    components(schemas(
        // Shared envelope types
        sakaloka_api::views::PageMeta,
        sakaloka_api::views::ListFilters,
        // Auth
        sakaloka_api::views::auth::LoginRequest,
        sakaloka_api::views::auth::RegisterRequest,
        sakaloka_api::views::auth::RefreshRequest,
        sakaloka_api::views::auth::LoginResponse,
        sakaloka_api::views::auth::UserProfile,
        sakaloka_api::views::auth::OrgSummary,
        sakaloka_api::views::auth::RoleSummary,
        // Product
        sakaloka_api::views::product::ProductResponse,
        sakaloka_api::views::product::ProductListResponse,
        sakaloka_api::views::product::CategorySummary,
        sakaloka_api::views::product::BrandSummary,
        sakaloka_api::views::product::VariantCount,
        sakaloka_api::views::product::CreateProductRequest,
        sakaloka_api::views::product::UpdateProductRequest,
        // Brand
        sakaloka_api::views::brand::BrandResponse,
        sakaloka_api::views::brand::CreateBrandRequest,
        sakaloka_api::views::brand::UpdateBrandRequest,
        // Category
        sakaloka_api::views::category::CategoryResponse,
        sakaloka_api::views::category::CreateCategoryRequest,
        sakaloka_api::views::category::UpdateCategoryRequest,
        // User
        sakaloka_api::views::user::UserResponse,
        sakaloka_api::views::user::CreateUserRequest,
        sakaloka_api::views::user::UpdateUserRequest,
        // Role
        sakaloka_api::views::role::RoleResponse,
        sakaloka_api::views::role::CreateRoleRequest,
        sakaloka_api::views::role::UpdateRoleRequest,
        sakaloka_api::views::role::PermissionActionResponse,
        sakaloka_api::views::role::PermissionModuleResponse,
        // Organization
        sakaloka_api::views::organization::OrganizationResponse,
        sakaloka_api::views::organization::CreateOrgRequest,
        sakaloka_api::views::organization::UpdateOrgRequest,
        sakaloka_api::views::organization::SelectOrgRequest,
        // Order / POS
        sakaloka_api::views::order::OrderResponse,
        sakaloka_api::views::order::OrderItemResponse,
        sakaloka_api::views::order::MenuResponse,
        sakaloka_api::views::order::CreateOrderRequest,
        sakaloka_api::views::order::CreateOrderItemRequest,
        sakaloka_api::views::order::UpdateOrderRequest,
        // Stock
        sakaloka_api::views::stock::StockResponse,
        sakaloka_api::views::stock::StockHistoryResponse,
        sakaloka_api::views::stock::AdjustStockRequest,
    ))
)]
struct ApiDoc;

fn main() {
    match ApiDoc::openapi().to_pretty_json() {
        Ok(spec) => println!("{spec}"),
        Err(e) => {
            eprintln!("Failed to generate OpenAPI JSON: {e}");
            std::process::exit(1);
        }
    }
}
