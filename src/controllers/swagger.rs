use loco_rs::prelude::*;
use utoipa::OpenApi;
use crate::controllers;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::sync::bootstrap,
        controllers::sync::incremental,
        controllers::sync::push,
    ),
    components(
        schemas(
            controllers::sync_dtos::PushPayloadDto,
            controllers::sync_dtos::PushPurchaseDto,
            controllers::sync_dtos::PushTransactionDto,
            controllers::sync_dtos::PushCategoryDto,
            controllers::sync_dtos::PushBrandDto,
            controllers::sync_dtos::PushDiscountDto,
            controllers::sync_dtos::PushProductDto,
            controllers::sync_dtos::PushProductPriceDto,
            controllers::sync_dtos::PushProductVariantDto,
            controllers::sync_dtos::PushCustomerDto,
            controllers::sync_dtos::PushSupplierDto,
            controllers::sync_dtos::PushReturnDto,
            controllers::sync_dtos::PushReturnItemDto,
            controllers::sync_dtos::PushPayableDto,
            controllers::sync_dtos::PushPayableRealizationDto,
            controllers::sync_dtos::PushReceivableDto,
            controllers::sync_dtos::PushReceivableRealizationDto,
            controllers::sync_dtos::PushStockOpnameDto,
            controllers::sync_dtos::PushStockOpnameItemDto,
            controllers::sync_dtos::PushPaymentMethodDto,
            controllers::sync_dtos::PushSalesTransactionDto,
            controllers::sync_dtos::PushSalesTransactionItemDto,
            controllers::sync_dtos::PushCashDrawerDto,
            controllers::sync_dtos::PushShiftDto,
            controllers::sync_dtos::PushFinanceDto
        )
    ),
    tags(
        (name = "sakaloka-universe", description = "Sakaloka Universe API")
    )
)]
struct ApiDoc;

pub async fn openapi_schema() -> Result<Response> {
    let schema = ApiDoc::openapi();
    format::json(schema)
}

use axum::response::Html;

pub async fn serve_swagger_ui() -> Html<String> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta name="description" content="SwaggerUI" />
  <title>SwaggerUI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui.css" />
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui-bundle.js" crossorigin></script>
<script src="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui-standalone-preset.js" crossorigin></script>
<script>
  window.onload = () => {
    window.ui = SwaggerUIBundle({
      url: '/swagger/openapi.json',
      dom_id: '#swagger-ui',
      presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIStandalonePreset
      ],
      layout: "StandaloneLayout",
    });
  };
</script>
</body>
</html>"#.to_string())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("swagger")
        .add("/openapi.json", get(openapi_schema))
        .add("/", get(serve_swagger_ui))
        .add("/index.html", get(serve_swagger_ui))
}
