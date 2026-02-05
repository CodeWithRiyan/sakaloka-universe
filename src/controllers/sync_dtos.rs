use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushPurchaseDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub supplier_id: String,
    pub total_amount: f64,
    pub payment_type: String, // 'CASH' | 'DEBT'
    pub status: Option<String>, // 'DRAFT' | 'COMPLETED'
    pub due_date: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushTransactionDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub product_id: String,
    pub r#type: String, // 'PURCHASE' | 'SALE' ...
    pub quantity: f64, // Number in TS, assuming float/double
    pub status: Option<String>,
    pub inventory_batch_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushReturnItemDto {
    pub id: Option<String>,
    pub product_id: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub purchase_return_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushReturnDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub supplier_id: String,
    pub total_amount: f64,
    pub return_type: String, // 'CASH' | 'ITEM'
    pub items: Option<Vec<PushReturnItemDto>>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushStockOpnameItemDto {
    pub id: Option<String>,
    pub product_id: String,
    pub quantity_system: f64,
    pub quantity_physical: f64,
    pub difference: f64,
    pub purchase_price: f64,
    pub financial_impact: f64,
    pub stock_opname_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushStockOpnameDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub date: String,
    pub note: Option<String>,
    pub status: String,
    pub total_gain: Option<f64>,
    pub total_loss: Option<f64>,
    pub created_by: String,
    pub items: Option<Vec<PushStockOpnameItemDto>>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushCategoryDto {
    pub id: String,
    pub name: String,
    pub point: Option<f64>,
    pub retail_point: Option<f64>,
    pub wholesale_point: Option<f64>,
    pub description: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushBrandDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushDiscountDto {
    pub id: String,
    pub name: String,
    pub nominal: f64,
    pub r#type: String,
    pub start_date: String,
    pub end_date: String,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushProductPriceDto {
    pub id: String,
    pub label: String,
    pub price: f64,
    pub minimum_purchase: f64,
    pub r#type: String,
    pub product_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushProductVariantDto {
    pub id: String,
    pub name: String,
    pub code: String,
    pub product_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushProductDto {
    pub id: String,
    pub name: String,
    pub barcode: Option<String>,
    pub purchase_price: Option<f64>,
    pub description: Option<String>,
    pub is_favorite: Option<bool>,
    pub is_active: Option<bool>,
    pub r#type: Option<String>,
    pub unit: Option<String>,
    pub minimum_stock: Option<f64>,
    pub category_id: String,
    pub brand_id: Option<String>,
    pub discount_id: Option<String>,
    pub supplier_id: Option<String>,
    pub prices: Option<Vec<PushProductPriceDto>>,
    pub variants: Option<Vec<PushProductVariantDto>>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushPaymentMethodDto {
    pub id: String,
    pub name: String,
    pub commission: f64,
    pub minimal_amount: f64,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushCustomerDto {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub code: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushSupplierDto {
    pub id: String,
    pub name: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushPayableDto {
    pub id: Option<String>,
    pub local_ref_id: Option<String>,
    pub nominal: f64,
    pub due_date: Option<String>,
    pub note: Option<String>,
    pub supplier_id: String,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushPayableRealizationDto {
    pub id: Option<String>,
    pub local_ref_id: Option<String>,
    pub payable_id: String,
    pub nominal: f64,
    pub realization_date: Option<String>,
    pub payment_method_id: String,
    pub note: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushReceivableDto {
    pub id: Option<String>,
    pub local_ref_id: Option<String>,
    pub nominal: f64,
    pub due_date: Option<String>,
    pub note: Option<String>,
    pub customer_id: String,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushReceivableRealizationDto {
    pub id: Option<String>,
    pub local_ref_id: Option<String>,
    pub receivable_id: String,
    pub nominal: f64,
    pub realization_date: Option<String>,
    pub payment_method_id: String,
    pub note: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushSalesTransactionItemDto {
    pub id: Option<String>,
    pub transaction_id: String,
    pub product_id: String,
    pub quantity: f64,
    pub sell_price: f64,
    pub note: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushSalesTransactionDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub customer_id: Option<String>,
    pub total_amount: f64,
    pub total_paid: f64,
    pub payment_type_id: String,
    pub transaction_date: String,
    pub status: String,
    pub note: Option<String>,
    pub items: Option<Vec<PushSalesTransactionItemDto>>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushCashDrawerDto {
    pub id: String,
    pub local_ref_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushShiftDto {
    pub id: Option<String>,
    pub local_ref_id: String,
    pub cash_drawer_id: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub initial_balance: f64,
    pub final_balance: Option<f64>,
    pub expected_balance: Option<f64>,
    pub difference: Option<f64>,
    pub status: String, // 'ACTIVE' | 'CLOSED'
    pub note: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushFinanceDto {
    pub id: String,
    pub local_ref_id: String,
    pub nominal: f64,
    pub r#type: String, // 'INCOME' | 'EXPENSES'
    pub expenses_type: Option<String>,
    pub transaction_date: String,
    pub status: String,
    pub note: Option<String>,
    pub input_to_cashdrawer: Option<bool>,
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushPayloadDto {
    pub purchases: Option<Vec<PushPurchaseDto>>,
    pub transactions: Option<Vec<PushTransactionDto>>,
    pub categories: Option<Vec<PushCategoryDto>>,
    pub brands: Option<Vec<PushBrandDto>>,
    pub discounts: Option<Vec<PushDiscountDto>>,
    pub products: Option<Vec<PushProductDto>>,
    pub customers: Option<Vec<PushCustomerDto>>,
    pub suppliers: Option<Vec<PushSupplierDto>>,
    pub purchase_returns: Option<Vec<PushReturnDto>>,
    pub payables: Option<Vec<PushPayableDto>>,
    pub payable_realizations: Option<Vec<PushPayableRealizationDto>>,
    pub receivables: Option<Vec<PushReceivableDto>>,
    pub receivable_realizations: Option<Vec<PushReceivableRealizationDto>>,
    pub stock_opnames: Option<Vec<PushStockOpnameDto>>,
    pub payment_methods: Option<Vec<PushPaymentMethodDto>>,
    pub sales_transactions: Option<Vec<PushSalesTransactionDto>>,
    pub cash_drawers: Option<Vec<PushCashDrawerDto>>,
    pub shifts: Option<Vec<PushShiftDto>>,
    pub finances: Option<Vec<PushFinanceDto>>,
}
