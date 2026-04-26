//! OpenAPI document definition shared by the Scalar UI and the `openapi` binary.

use utoipa::OpenApi;

/// Root OpenAPI document aggregating all schemas and paths.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sakaloka API",
        version = env!("SAKALOKA_VERSION"),
        description = "Sakaloka-Universe Earth REST API"
    ),
    modifiers(&SecurityAddon),
    paths(
        // Auth
        crate::controllers::auth::handlers::login,
        crate::controllers::auth::handlers::register,
        crate::controllers::auth::handlers::refresh,
        crate::controllers::auth::handlers::profile,
        // Product
        crate::controllers::product::handlers::list,
        crate::controllers::product::handlers::show,
        crate::controllers::product::handlers::create,
        crate::controllers::product::handlers::update,
        crate::controllers::product::handlers::remove,
        // Brand
        crate::controllers::brand::handlers::list,
        crate::controllers::brand::handlers::show,
        crate::controllers::brand::handlers::create,
        crate::controllers::brand::handlers::update,
        crate::controllers::brand::handlers::remove,
        // Category
        crate::controllers::category::handlers::list,
        crate::controllers::category::handlers::show,
        crate::controllers::category::handlers::create,
        crate::controllers::category::handlers::update,
        crate::controllers::category::handlers::remove,
        // User
        crate::controllers::user::handlers::list,
        crate::controllers::user::handlers::show,
        crate::controllers::user::handlers::create,
        crate::controllers::user::handlers::update,
        crate::controllers::user::handlers::remove,
        // Role
        crate::controllers::role::handlers::list,
        crate::controllers::role::handlers::permissions,
        crate::controllers::role::handlers::show,
        crate::controllers::role::handlers::create,
        crate::controllers::role::handlers::update,
        crate::controllers::role::handlers::remove,
        // Organization
        crate::controllers::organization::handlers::list,
        crate::controllers::organization::handlers::current,
        crate::controllers::organization::handlers::select,
        crate::controllers::organization::handlers::show,
        crate::controllers::organization::handlers::create,
        crate::controllers::organization::handlers::update,
        crate::controllers::organization::handlers::remove,
        // POS
        crate::controllers::pos::handlers::menu,
        crate::controllers::pos::handlers::list_orders,
        crate::controllers::pos::handlers::active_orders,
        crate::controllers::pos::handlers::order_history,
        crate::controllers::pos::handlers::show_order,
        crate::controllers::pos::handlers::create_order,
        crate::controllers::pos::handlers::update_order,
        // Stock
        crate::controllers::stock::handlers::list,
        crate::controllers::stock::handlers::low_stock,
        crate::controllers::stock::handlers::show,
        crate::controllers::stock::handlers::history,
        crate::controllers::stock::handlers::adjust,
        // BOM
        crate::controllers::bom::handlers::list,
        crate::controllers::bom::handlers::show,
        crate::controllers::bom::handlers::show_by_product,
        crate::controllers::bom::handlers::create,
        crate::controllers::bom::handlers::update,
        crate::controllers::bom::handlers::remove,
        crate::controllers::bom::handlers::activate,
        crate::controllers::bom::handlers::list_items,
        crate::controllers::bom::handlers::add_item,
        crate::controllers::bom::handlers::update_item,
        crate::controllers::bom::handlers::remove_item,
        crate::controllers::bom::handlers::list_production_runs,
        crate::controllers::bom::handlers::show_production_run,
        crate::controllers::bom::handlers::create_production_run,
        crate::controllers::bom::handlers::update_production_run,
        crate::controllers::bom::handlers::add_consumption,
        crate::controllers::bom::handlers::list_consumptions,
    ),
    components(schemas(
        crate::views::ErrorResponse,
        crate::views::PageMeta,
        crate::views::ListFilters,
        // Auth
        crate::views::auth::LoginRequest,
        crate::views::auth::RegisterRequest,
        crate::views::auth::RefreshRequest,
        crate::views::auth::LoginResponse,
        crate::views::auth::UserProfile,
        crate::views::auth::OrgSummary,
        crate::views::auth::RoleSummary,
        // Product
        crate::views::product::ProductResponse,
        crate::views::product::ProductListResponse,
        crate::views::product::CategorySummary,
        crate::views::product::BrandSummary,
        crate::views::product::VariantCount,
        crate::views::product::CreateProductRequest,
        crate::views::product::UpdateProductRequest,
        // Brand
        crate::views::brand::BrandResponse,
        crate::views::brand::CreateBrandRequest,
        crate::views::brand::UpdateBrandRequest,
        // Category
        crate::views::category::CategoryResponse,
        crate::views::category::CreateCategoryRequest,
        crate::views::category::UpdateCategoryRequest,
        // User
        crate::views::user::UserResponse,
        crate::views::user::CreateUserRequest,
        crate::views::user::UpdateUserRequest,
        // Role
        crate::views::role::RoleResponse,
        crate::views::role::CreateRoleRequest,
        crate::views::role::UpdateRoleRequest,
        crate::views::role::PermissionActionResponse,
        crate::views::role::PermissionModuleResponse,
        // Organization
        crate::views::organization::OrganizationResponse,
        crate::views::organization::CreateOrgRequest,
        crate::views::organization::UpdateOrgRequest,
        crate::views::organization::SelectOrgRequest,
        // Order / POS
        crate::views::order::OrderResponse,
        crate::views::order::OrderItemResponse,
        crate::views::order::MenuResponse,
        crate::views::order::CreateOrderRequest,
        crate::views::order::CreateOrderItemRequest,
        crate::views::order::UpdateOrderRequest,
        // Stock
        crate::views::stock::StockResponse,
        crate::views::stock::StockHistoryResponse,
        crate::views::stock::AdjustStockRequest,
        // BOM
        crate::views::bom::BomResponse,
        crate::views::bom::BomItemResponse,
        crate::views::bom::BomWithItemsResponse,
        crate::views::bom::ProductionRunResponse,
        crate::views::bom::ConsumptionResponse,
        crate::views::bom::ProductionRunWithConsumptionResponse,
        crate::views::bom::CreateBomRequest,
        crate::views::bom::UpdateBomRequest,
        crate::views::bom::CreateBomItemRequest,
        crate::views::bom::UpdateBomItemRequest,
        crate::views::bom::CreateProductionRunRequest,
        crate::views::bom::UpdateProductionRunRequest,
        crate::views::bom::CreateConsumptionRequest,
        crate::views::bom::BomCostBreakdownResponse,
        crate::views::bom::BomItemCostResponse,
    ))
)]
pub struct ApiDoc;

/// Adds Bearer token security scheme to the OpenAPI document.
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}
