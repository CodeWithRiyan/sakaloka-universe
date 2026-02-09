use loco_rs::prelude::*;
use sea_orm::{sea_query::OnConflict, Condition};
use serde_json::json;
use chrono::{NaiveDateTime, Utc};
use crate::controllers::sync_dtos::PushPayloadDto;
use std::collections::HashMap;

use crate::models::_entities::{
    brands, categories, customers, discounts, finances, inventory_transactions,
    payable_realizations, payables, payment_methods, product_prices, 
    products, purchase_return_items, purchase_returns, purchases, 
    receivable_realizations, receivables, stock_opnames, stock_opname_items,
    suppliers, transaction_items, transactions, shifts, cash_drawers,
    sea_orm_active_enums::*,
};

pub struct Sync;

impl Sync {
    pub async fn incremental(
        db: &DatabaseConnection,
        org_id: &str,
        since: NaiveDateTime,
    ) -> Result<serde_json::Value> {
        
        let categories = categories::Entity::find()
             .filter(Condition::all()
                .add(categories::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(categories::Column::UpdatedAt.gt(since)).add(categories::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;
        
        let brands = brands::Entity::find()
             .filter(Condition::all()
                .add(brands::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(brands::Column::UpdatedAt.gt(since)).add(brands::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let discounts = discounts::Entity::find()
             .filter(Condition::all()
                .add(discounts::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(discounts::Column::UpdatedAt.gt(since)).add(discounts::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;
             
        let products = products::Entity::find()
             .filter(Condition::all()
                .add(products::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(products::Column::UpdatedAt.gt(since)).add(products::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let prices = product_prices::Entity::find()
             .filter(Condition::all()
                .add(product_prices::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(product_prices::Column::UpdatedAt.gt(since)).add(product_prices::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let customers = customers::Entity::find()
             .filter(Condition::all()
                .add(customers::Column::OrganizationId.eq(org_id)) 
                .add(Condition::any().add(customers::Column::UpdatedAt.gt(since)).add(customers::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let suppliers = suppliers::Entity::find()
             .filter(Condition::all()
                .add(suppliers::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(suppliers::Column::UpdatedAt.gt(since)).add(suppliers::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;
        
        let purchases = purchases::Entity::find()
             .filter(Condition::all()
                .add(purchases::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(purchases::Column::UpdatedAt.gt(since)).add(purchases::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let inventory_transactions = inventory_transactions::Entity::find()
             .filter(Condition::all()
                .add(inventory_transactions::Column::OrganizationId.eq(org_id))
                .add(Condition::any()
                    .add(inventory_transactions::Column::CreatedAt.gt(since))
                    .add(inventory_transactions::Column::DeletedAt.gt(since))
                )
             )
             .all(db).await?;

        let purchase_returns = purchase_returns::Entity::find()
             .filter(Condition::all()
                 .add(purchase_returns::Column::OrganizationId.eq(org_id))
                 .add(purchase_returns::Column::UpdatedAt.gt(since))
             )
             .find_with_related(purchase_return_items::Entity)
             .all(db).await?;
        
        let purchase_returns_json: Vec<serde_json::Value> = purchase_returns.into_iter().map(|(parent, items)| {
            let mut val = serde_json::to_value(parent).unwrap();
            if let Some(obj) = val.as_object_mut() {
                obj.insert("items".to_string(), serde_json::to_value(items).unwrap());
            }
            val
        }).collect();


        let stock_opnames = stock_opnames::Entity::find()
             .filter(Condition::all()
                .add(stock_opnames::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(stock_opnames::Column::UpdatedAt.gt(since)).add(stock_opnames::Column::DeletedAt.gt(since)))
             )
             .find_with_related(stock_opname_items::Entity)
             .all(db).await?;
        
        let stock_opnames_json: Vec<serde_json::Value> = stock_opnames.into_iter().map(|(parent, items)| {
            let mut val = serde_json::to_value(parent).unwrap();
            if let Some(obj) = val.as_object_mut() {
                obj.insert("items".to_string(), serde_json::to_value(items).unwrap());
            }
            val
        }).collect();

        let payment_methods = payment_methods::Entity::find()
             .filter(Condition::all()
                .add(payment_methods::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(payment_methods::Column::UpdatedAt.gt(since)).add(payment_methods::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let payables = payables::Entity::find()
             .filter(Condition::all()
                .add(payables::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(payables::Column::UpdatedAt.gt(since)).add(payables::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;
        
        let payable_realizations = payable_realizations::Entity::find()
             .filter(Condition::all()
                .add(payable_realizations::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(payable_realizations::Column::UpdatedAt.gt(since)).add(payable_realizations::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let receivables = receivables::Entity::find()
             .filter(Condition::all()
                .add(receivables::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(receivables::Column::UpdatedAt.gt(since)).add(receivables::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let receivable_realizations = receivable_realizations::Entity::find()
             .filter(Condition::all()
                .add(receivable_realizations::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(receivable_realizations::Column::UpdatedAt.gt(since)).add(receivable_realizations::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        let finances = finances::Entity::find()
             .filter(Condition::all()
                .add(finances::Column::OrganizationId.eq(org_id))
                .add(Condition::any().add(finances::Column::UpdatedAt.gt(since)).add(finances::Column::DeletedAt.gt(since)))
             )
             .all(db).await?;

        Ok(json!({
            "data": {
                "categories": categories,
                "brands": brands,
                "discounts": discounts,
                "products": products,
                "prices": prices,
                "customers": customers,
                "suppliers": suppliers,
                "purchases": purchases,
                "transactions": inventory_transactions,
                "purchaseReturns": purchase_returns_json,
                "stockOpnames": stock_opnames_json,
                "paymentMethods": payment_methods,
                "payables": payables,
                "payableRealizations": payable_realizations,
                "receivables": receivables,
                "receivableRealizations": receivable_realizations,
                "finances": finances
            },
            "timestamp": Utc::now().to_rfc3339()
        }))
    }

    pub async fn push(
        db: &DatabaseConnection,
        org_id: &str,
        _user_id: &str,
        payload: PushPayloadDto,
    ) -> Result<serde_json::Value> {
        let mut results = HashMap::new();

        fn parse_date_opt(s: Option<String>) -> Option<NaiveDateTime> {
             s.and_then(|d| NaiveDateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M:%S%.fZ").ok()
               .or_else(|| NaiveDateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M:%S").ok()))
        }
        
        // 1. Payment Methods
        if let Some(items) = payload.payment_methods {
            let mut ids = Vec::new();
            for item in items {
                let am = payment_methods::ActiveModel {
                    id: Set(item.id.clone()),
                    name: Set(item.name),
                    commission: Set(item.commission),
                    minimal_amount: Set(item.minimal_amount),
                    organization_id: Set(org_id.to_string()),
                    updated_at: Set(Utc::now().naive_utc()),
                    created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                    deleted_at: Set(parse_date_opt(item.deleted_at)),
                    ..Default::default()
                };
                payment_methods::Entity::insert(am)
                    .on_conflict(
                        OnConflict::column(payment_methods::Column::Id)
                            .update_columns([
                                payment_methods::Column::Name,
                                payment_methods::Column::Commission,
                                payment_methods::Column::MinimalAmount,
                                payment_methods::Column::UpdatedAt,
                                payment_methods::Column::DeletedAt,
                            ])
                            .to_owned(),
                    )
                    .exec(db).await.ok();
                ids.push(json!({"id": item.id, "server_id": item.id}));
            }
            results.insert("paymentMethods", ids);
        }

        // 2. Categories
        if let Some(items) = payload.categories {
            let mut ids = Vec::new();
            for item in items {
                let am = categories::ActiveModel {
                    id: Set(item.id.clone()),
                    name: Set(item.name),
                    retail_point: Set(item.retail_point.unwrap_or(0.0)),
                    wholesale_point: Set(item.wholesale_point.unwrap_or(0.0)),
                    description: Set(item.description),
                    organization_id: Set(org_id.to_string()),
                    updated_at: Set(Utc::now().naive_utc()),
                    created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                    deleted_at: Set(parse_date_opt(item.deleted_at)),
                    ..Default::default()
                };
                categories::Entity::insert(am)
                    .on_conflict(
                        OnConflict::column(categories::Column::Id)
                            .update_columns([
                                categories::Column::Name, 
                                categories::Column::RetailPoint,
                                categories::Column::WholesalePoint,
                                categories::Column::Description,
                                categories::Column::UpdatedAt,
                                categories::Column::DeletedAt
                            ])
                            .to_owned()
                    )
                    .exec(db).await.ok();
                ids.push(json!({"id": item.id, "server_id": item.id}));
            }
             results.insert("categories", ids);
        }

        // 3. Customers
        if let Some(items) = payload.customers {
             let mut ids = Vec::new();
             for item in items {
                 let category_enum = match item.category.as_deref() {
                     Some("WHOLESALE") => CustomerCategory::Wholesale,
                     Some("RETAIL") => CustomerCategory::Retail,
                     _ => CustomerCategory::Retail,
                 };

                 let am = customers::ActiveModel {
                     id: Set(item.id.clone()),
                     name: Set(item.name),
                     category: Set(category_enum),
                     code: Set(item.code),
                     phone: Set(item.phone),
                     address: Set(item.address),
                     organization_id: Set(Some(org_id.to_string())),
                     updated_at: Set(Utc::now().naive_utc()),
                     created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                     deleted_at: Set(parse_date_opt(item.deleted_at)),  
                 };
                 customers::Entity::insert(am)
                    .on_conflict(
                        OnConflict::column(customers::Column::Id)
                            .update_columns([
                                customers::Column::Name,
                                customers::Column::Category,
                                customers::Column::Code,
                                customers::Column::Phone,
                                customers::Column::Address,
                                customers::Column::UpdatedAt,
                                customers::Column::DeletedAt
                            ]).to_owned()
                    ).exec(db).await.ok();
                 ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("customers", ids);
        }

         // 4. Products & Prices
         if let Some(items) = payload.products {
              let mut ids = Vec::new();
              for item in items {
                  let type_enum = match item.r#type.as_deref() {
                      Some("VARIANTS") => ProductType::Variants,
                      Some("MULTIUNIT") => ProductType::Multiunit,
                      _ => ProductType::Default,
                  };

                  let am = products::ActiveModel {
                      id: Set(item.id.clone()),
                      name: Set(item.name),
                      description: Set(item.description),
                      barcode: Set(item.barcode),
                      purchase_price: Set(item.purchase_price.unwrap_or(0.0)),
                      is_favorite: Set(item.is_favorite.unwrap_or(false)),
                      is_active: Set(item.is_active.unwrap_or(true)),
                      r#type: Set(type_enum),
                      unit: Set(item.unit),
                      minimum_stock: Set(item.minimum_stock.unwrap_or(0.0)),
                      category_id: Set(item.category_id),
                      brand_id: Set(item.brand_id),
                      discount_id: Set(item.discount_id),
                      supplier_id: Set(item.supplier_id),
                      organization_id: Set(org_id.to_string()),
                      updated_at: Set(Utc::now().naive_utc()),
                      created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                      deleted_at: Set(parse_date_opt(item.deleted_at)),
                  };
                  products::Entity::insert(am)
                      .on_conflict(OnConflict::column(products::Column::Id).update_columns([
                          products::Column::Name, products::Column::Description, products::Column::Barcode,
                          products::Column::PurchasePrice, products::Column::IsFavorite, products::Column::IsActive,
                          products::Column::Type, products::Column::Unit, products::Column::MinimumStock,
                          products::Column::CategoryId, products::Column::BrandId, products::Column::DiscountId,
                          products::Column::SupplierId, products::Column::UpdatedAt, products::Column::DeletedAt
                      ]).to_owned())
                      .exec(db).await.ok();
                  
                  if let Some(prices) = item.prices {
                      for p in prices {
                          let pam = product_prices::ActiveModel {
                              id: Set(p.id),
                              label: Set(p.label),
                              price: Set(p.price),
                              minimum_purchase: Set(p.minimum_purchase),
                              product_id: Set(item.id.clone()),
                              organization_id: Set(org_id.to_string()),
                              updated_at: Set(Utc::now().naive_utc()),
                              created_at: Set(Utc::now().naive_utc()), 
                              ..Default::default()
                          };
                          product_prices::Entity::insert(pam).on_conflict(
                              OnConflict::column(product_prices::Column::Id).update_columns([product_prices::Column::Price, product_prices::Column::Label, product_prices::Column::MinimumPurchase]).to_owned()
                          ).exec(db).await.ok();
                      }
                  }
                  ids.push(json!({"id": item.id, "server_id": item.id}));
              }
              results.insert("products", ids);
         }
        
        // 5. Finances
        if let Some(items) = payload.finances {
             let mut ids = Vec::new();
             for item in items {
                  let status_enum = match item.status.as_str() { "COMPLETED" => FinanceStatus::Completed, _ => FinanceStatus::Draft };
                  let type_enum = match item.r#type.as_str() { "INCOME" => FinanceType::Income, "EXPENSES" => FinanceType::Expenses, _ => FinanceType::Income };
                  let expenses_type_enum = item.expenses_type.as_deref().map(|s| match s { "STORE_EXPENSES" => FinanceExpensesType::StoreExpenses, "SUPPLIES" => FinanceExpensesType::Supplies, "EQUIPMENT" => FinanceExpensesType::Equipment, _ => FinanceExpensesType::StoreExpenses });

                  let am = finances::ActiveModel {
                      id: Set(item.id.clone()),
                      local_ref_id: Set(item.local_ref_id),
                      nominal: Set(item.nominal),
                      r#type: Set(type_enum),
                      expenses_type: Set(expenses_type_enum),
                      transaction_date: Set(Utc::now().naive_utc()),
                      status: Set(status_enum),
                      note: Set(item.note),
                      input_to_cashdrawer: Set(item.input_to_cashdrawer.unwrap_or(false)),
                      organization_id: Set(org_id.to_string()),
                      user_id: Set(item.user_id),
                      created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                      updated_at: Set(Utc::now().naive_utc()),
                      deleted_at: Set(parse_date_opt(item.deleted_at)),  
                  };
                  finances::Entity::insert(am)
                       .on_conflict(OnConflict::column(finances::Column::Id).update_columns([finances::Column::Nominal, finances::Column::Status, finances::Column::Note, finances::Column::UpdatedAt, finances::Column::DeletedAt]).to_owned())
                       .exec(db).await.ok();
                  ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("finances", ids);
        }

        // 6. Brands
        if let Some(items) = payload.brands {
             let mut ids = Vec::new();
             for item in items {
                 let am = brands::ActiveModel {
                     id: Set(item.id.clone()),
                     name: Set(item.name),
                     description: Set(item.description),
                     organization_id: Set(org_id.to_string()),
                     updated_at: Set(Utc::now().naive_utc()),
                     created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                     deleted_at: Set(parse_date_opt(item.deleted_at)),
                     ..Default::default()
                 };
                 brands::Entity::insert(am).on_conflict(OnConflict::column(brands::Column::Id).update_columns([brands::Column::Name, brands::Column::Description, brands::Column::UpdatedAt, brands::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                 ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("brands", ids);
        }

        // 7. Discounts
        if let Some(items) = payload.discounts {
             let mut ids = Vec::new();
             for item in items {
                 let type_enum = match item.r#type.as_str() { "PERCENTAGE" => DiscountType::Percentage, "FLAT" => DiscountType::Flat, _ => DiscountType::Flat };
                 let am = discounts::ActiveModel { id: Set(item.id.clone()), name: Set(item.name), nominal: Set(item.nominal), r#type: Set(type_enum), start_date: Set(Utc::now().naive_utc()), end_date: Set(Utc::now().naive_utc()), organization_id: Set(org_id.to_string()), updated_at: Set(Utc::now().naive_utc()), created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())), deleted_at: Set(parse_date_opt(item.deleted_at)), ..Default::default() };
                 discounts::Entity::insert(am).on_conflict(OnConflict::column(discounts::Column::Id).update_columns([discounts::Column::Name, discounts::Column::Nominal, discounts::Column::UpdatedAt, discounts::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                 ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("discounts", ids);
        }

        // 8. Suppliers
        if let Some(items) = payload.suppliers {
             let mut ids = Vec::new();
             for item in items {
                 let am = suppliers::ActiveModel { id: Set(item.id.clone()), name: Set(item.name), phone: Set(item.phone), address: Set(item.address), organization_id: Set(org_id.to_string()), updated_at: Set(Utc::now().naive_utc()), created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())), deleted_at: Set(parse_date_opt(item.deleted_at)), ..Default::default() };
                 suppliers::Entity::insert(am).on_conflict(OnConflict::column(suppliers::Column::Id).update_columns([suppliers::Column::Name, suppliers::Column::Phone, suppliers::Column::Address, suppliers::Column::UpdatedAt, suppliers::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                 ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("suppliers", ids);
        }

        // 9. Purchases
        if let Some(items) = payload.purchases {
            let mut ids = Vec::new();
            for item in items {
                let status_enum = match item.status.as_deref() { Some("COMPLETED") => PurchaseStatus::Completed, _ => PurchaseStatus::Draft };
                let am = purchases::ActiveModel {
                    id: Set(item.id.clone().unwrap_or_default()), // ID might be optional in DTO but needed for upsert? Actually DTO has id: Option<String>.
                    // If ID is missing, we use local_ref_id? Or generate new?
                    // Usually push sends ID. If not, we can't upsert reliably without checking local_ref_id.
                    // Assuming ID is present or we skip.
                    // Let's assume ID is present for now as per other entities. item.id.clone() is Option.
                    // Wait, if item.id is None, Set(None) fails for primary key.
                    // We need strict ID. If None, skip?
                    // NestJS upserts by ID.
                    local_ref_id: Set(Some(item.local_ref_id)),
                    supplier_id: Set(item.supplier_id),                         
                    total_amount: Set(item.total_amount),
                    payment_type: Set(match item.payment_type.as_str() { "CASH" => PaymentType::Cash, "DEBT" => PaymentType::Debt, _ => PaymentType::Cash }),
                    status: Set(status_enum),
                    due_date: Set(parse_date_opt(item.due_date)),
                    organization_id: Set(org_id.to_string()),
                    updated_at: Set(Utc::now().naive_utc()),
                    created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                    deleted_at: Set(parse_date_opt(item.deleted_at)),
                    ..Default::default()
                };
                if let Some(id) = item.id {
                    let mut am = am;
                    am.id = Set(id.clone());
                    purchases::Entity::insert(am).on_conflict(OnConflict::column(purchases::Column::Id).update_columns([purchases::Column::LocalRefId, purchases::Column::SupplierId, purchases::Column::TotalAmount, purchases::Column::PaymentType, purchases::Column::Status, purchases::Column::DueDate, purchases::Column::UpdatedAt, purchases::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                    ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("purchases", ids);
        }

        // 10. Inventory Transactions
        if let Some(items) = payload.transactions {
            let mut ids = Vec::new();
            for item in items {
                let type_enum = match item.r#type.as_str() { "PURCHASE" => InventoryTransactionType::Purchase, "SALE" => InventoryTransactionType::Sale, "RETURN_PURCHASE" => InventoryTransactionType::ReturnPurchase, "STOCK_OPNAME" => InventoryTransactionType::StockOpname, "ADJUSTMENT" => InventoryTransactionType::Adjustment, _ => InventoryTransactionType::Sale };
                // item.id is Option.
                if let Some(id) = item.id {
                     let am = inventory_transactions::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(Some(item.local_ref_id)),
                         product_id: Set(item.product_id),
                         r#type: Set(type_enum),
                         quantity: Set(item.quantity),
                         status: Set(match item.status.as_deref() { Some("COMPLETED") => PurchaseStatus::Completed, _ => PurchaseStatus::Draft }), // Mapped to PurchaseStatus
                         inventory_batch_id: Set(item.inventory_batch_id),
                         organization_id: Set(org_id.to_string()),
                         // updated_at removed
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         deleted_at: Set(parse_date_opt(item.deleted_at)),
                     };
                     inventory_transactions::Entity::insert(am).on_conflict(OnConflict::column(inventory_transactions::Column::Id).update_columns([inventory_transactions::Column::Quantity, inventory_transactions::Column::Status, inventory_transactions::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                     ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("transactions", ids);
        }

        // 11. Purchase Returns
        if let Some(items) = payload.purchase_returns {
            let mut ids = Vec::new();
            for item in items {
                 if let Some(id) = item.id {
                     let type_enum = match item.return_type.as_str() { "CASH" => PurchaseReturnType::Cash, "ITEM" => PurchaseReturnType::Item, _ => PurchaseReturnType::Cash };
                     let am = purchase_returns::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(Some(item.local_ref_id)),
                         supplier_id: Set(item.supplier_id),
                         total_amount: Set(item.total_amount),
                         return_type: Set(type_enum),
                         organization_id: Set(org_id.to_string()),
                         updated_at: Set(Utc::now().naive_utc()),
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         // No deleted_at
                     };
                     purchase_returns::Entity::insert(am).on_conflict(OnConflict::column(purchase_returns::Column::Id).update_columns([purchase_returns::Column::TotalAmount, purchase_returns::Column::UpdatedAt]).to_owned()).exec(db).await.ok();
                     
                     if let Some(p_items) = item.items {
                         for p_item in p_items {
                             if let Some(pid) = p_item.id {
                                 let iam = purchase_return_items::ActiveModel {
                                     id: Set(pid),
                                     product_id: Set(p_item.product_id),
                                     quantity: Set(p_item.quantity),
                                     purchase_price: Set(p_item.purchase_price),
                                     purchase_return_id: Set(id.clone()),
                                     organization_id: Set(org_id.to_string()),
                                     // No timestamps
                                 };
                                 purchase_return_items::Entity::insert(iam).on_conflict(OnConflict::column(purchase_return_items::Column::Id).update_columns([purchase_return_items::Column::Quantity, purchase_return_items::Column::PurchasePrice]).to_owned()).exec(db).await.ok();
                             }
                         }
                     }
                     ids.push(json!({"id": id, "server_id": id}));
                 }
            }
            results.insert("purchaseReturns", ids);
        }

        // 12. Stock Opnames
        if let Some(items) = payload.stock_opnames {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                    let status_enum = match item.status.as_str() { "DIFFERENCE" => StockOpnameStatus::Difference, "DONE" => StockOpnameStatus::Done, _ => StockOpnameStatus::Done };
                    let am = stock_opnames::ActiveModel {
                        id: Set(id.clone()),
                        local_ref_id: Set(Some(item.local_ref_id)),
                        date: Set(NaiveDateTime::parse_from_str(&item.date, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap_or_else(|_| Utc::now().naive_utc())),
                        note: Set(item.note),
                        status: Set(status_enum),
                        total_gain: Set(item.total_gain.unwrap_or(0.0)),
                        total_loss: Set(item.total_loss.unwrap_or(0.0)),
                        created_by: Set(Some(item.created_by)),
                        organization_id: Set(org_id.to_string()),
                        updated_at: Set(Utc::now().naive_utc()),
                        created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                        deleted_at: Set(parse_date_opt(item.deleted_at)),
                    };
                    stock_opnames::Entity::insert(am).on_conflict(OnConflict::column(stock_opnames::Column::Id).update_columns([stock_opnames::Column::Status, stock_opnames::Column::TotalGain, stock_opnames::Column::TotalLoss, stock_opnames::Column::UpdatedAt, stock_opnames::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                    
                    if let Some(s_items) = item.items {
                        for s_item in s_items {
                            if let Some(sid) = s_item.id {
                                let iam = stock_opname_items::ActiveModel {
                                    id: Set(sid),
                                    product_id: Set(s_item.product_id),
                                    quantity_system: Set(s_item.quantity_system),
                                    quantity_physical: Set(s_item.quantity_physical),
                                    difference: Set(s_item.difference),
                                    purchase_price: Set(s_item.purchase_price),
                                    financial_impact: Set(s_item.financial_impact),
                                    stock_opname_id: Set(id.clone()),
                                    organization_id: Set(org_id.to_string()),
                                    // No timestamps
                                };
                                stock_opname_items::Entity::insert(iam).on_conflict(OnConflict::column(stock_opname_items::Column::Id).update_columns([stock_opname_items::Column::QuantityPhysical, stock_opname_items::Column::Difference]).to_owned()).exec(db).await.ok();
                            }
                        }
                    }
                    ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("stockOpnames", ids);
        }

        // 13. Payables
        if let Some(items) = payload.payables {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                    let am = payables::ActiveModel {
                        id: Set(id.clone()),
                        local_ref_id: Set(item.local_ref_id),
                        nominal: Set(item.nominal),
                        due_date: Set(parse_date_opt(item.due_date)),
                        note: Set(item.note),
                        supplier_id: Set(item.supplier_id),
                        organization_id: Set(org_id.to_string()),
                        updated_at: Set(Utc::now().naive_utc()),
                        created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                        deleted_at: Set(parse_date_opt(item.deleted_at)),
                        ..Default::default()
                    };
                    payables::Entity::insert(am).on_conflict(OnConflict::column(payables::Column::Id).update_columns([payables::Column::Nominal, payables::Column::DueDate, payables::Column::UpdatedAt, payables::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                    ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("payables", ids);
        }

        // 14. Payable Realizations
        if let Some(items) = payload.payable_realizations {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                     let am = payable_realizations::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(item.local_ref_id),
                         payable_id: Set(item.payable_id),
                         nominal: Set(item.nominal),
                         realization_date: Set(parse_date_opt(item.realization_date).unwrap_or_else(|| Utc::now().naive_utc())),
                         payment_method_id: Set(item.payment_method_id),
                         note: Set(item.note),
                         organization_id: Set(org_id.to_string()),
                         updated_at: Set(Utc::now().naive_utc()),
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         deleted_at: Set(parse_date_opt(item.deleted_at)),
                     };
                     payable_realizations::Entity::insert(am).on_conflict(OnConflict::column(payable_realizations::Column::Id).update_columns([payable_realizations::Column::Nominal, payable_realizations::Column::UpdatedAt, payable_realizations::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                     ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("payableRealizations", ids);
        }

        // 15. Receivables
        if let Some(items) = payload.receivables {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                     // Using item.customer_id to populate user_id as per entity definition analysis
                     let am = receivables::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(item.local_ref_id),
                         nominal: Set(item.nominal),
                         due_date: Set(parse_date_opt(item.due_date)),
                         note: Set(item.note),
                         user_id: Set(item.customer_id), // Mapping customer_id -> user_id
                         organization_id: Set(org_id.to_string()),
                         updated_at: Set(Utc::now().naive_utc()),
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         deleted_at: Set(parse_date_opt(item.deleted_at)),
                         ..Default::default()
                     };
                     receivables::Entity::insert(am).on_conflict(OnConflict::column(receivables::Column::Id).update_columns([receivables::Column::Nominal, receivables::Column::DueDate, receivables::Column::UpdatedAt, receivables::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                     ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("receivables", ids);
        }

        // 16. Receivable Realizations
        if let Some(items) = payload.receivable_realizations {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                     let am = receivable_realizations::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(item.local_ref_id),
                         receivable_id: Set(item.receivable_id),
                         nominal: Set(item.nominal),
                         realization_date: Set(parse_date_opt(item.realization_date).unwrap_or_else(|| Utc::now().naive_utc())),
                         payment_method_id: Set(item.payment_method_id),
                         note: Set(item.note),
                         organization_id: Set(org_id.to_string()),
                         updated_at: Set(Utc::now().naive_utc()),
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         deleted_at: Set(parse_date_opt(item.deleted_at)),
                     };
                     receivable_realizations::Entity::insert(am).on_conflict(OnConflict::column(receivable_realizations::Column::Id).update_columns([receivable_realizations::Column::Nominal, receivable_realizations::Column::UpdatedAt, receivable_realizations::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                     ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("receivableRealizations", ids);
        }

        // 17. Sales Transactions
        if let Some(items) = payload.sales_transactions {
            let mut ids = Vec::new();
            for item in items {
                if let Some(id) = item.id {
                     let status_enum = match item.status.as_str() { "DRAFT" => TransactionStatus::Draft, "COMPLETED" => TransactionStatus::Completed, _ => TransactionStatus::Completed };
                     let am = transactions::ActiveModel {
                         id: Set(id.clone()),
                         local_ref_id: Set(item.local_ref_id),
                         customer_id: Set(item.customer_id),
                         total_amount: Set(item.total_amount),
                         total_paid: Set(item.total_paid),
                         payment_type_id: Set(item.payment_type_id),
                         transaction_date: Set(NaiveDateTime::parse_from_str(&item.transaction_date, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap_or_else(|_| Utc::now().naive_utc())),
                         status: Set(status_enum),
                         note: Set(item.note),
                         organization_id: Set(org_id.to_string()),
                         updated_at: Set(Utc::now().naive_utc()),
                         created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                         deleted_at: Set(parse_date_opt(item.deleted_at)),
                     };
                     transactions::Entity::insert(am).on_conflict(OnConflict::column(transactions::Column::Id).update_columns([transactions::Column::TotalAmount, transactions::Column::TotalPaid, transactions::Column::Status, transactions::Column::UpdatedAt, transactions::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                     
                     if let Some(t_items) = item.items {
                         for t_item in t_items {
                             if let Some(tid) = t_item.id {
                                 let iam = transaction_items::ActiveModel {
                                     id: Set(tid),
                                     transaction_id: Set(id.clone()),
                                     product_id: Set(t_item.product_id),
                                     quantity: Set(t_item.quantity),
                                     sell_price: Set(t_item.sell_price),
                                     note: Set(t_item.note),
                                     organization_id: Set(org_id.to_string()),
                                     updated_at: Set(Utc::now().naive_utc()),
                                     created_at: Set(parse_date_opt(t_item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                                     deleted_at: Set(parse_date_opt(t_item.deleted_at)),
                                 };
                                 transaction_items::Entity::insert(iam).on_conflict(OnConflict::column(transaction_items::Column::Id).update_columns([transaction_items::Column::Quantity, transaction_items::Column::SellPrice, transaction_items::Column::UpdatedAt, transaction_items::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                             }
                         }
                     }
                     ids.push(json!({"id": id, "server_id": id}));
                }
            }
            results.insert("salesTransactions", ids);
        }

        // 18. Cash Drawers
        if let Some(items) = payload.cash_drawers {
             let mut ids = Vec::new();
             for item in items {
                  let am = cash_drawers::ActiveModel {
                      id: Set(item.id.clone()),
                      local_ref_id: Set(item.local_ref_id),
                      name: Set(item.name),
                      description: Set(item.description),
                      is_active: Set(item.is_active),
                      organization_id: Set(org_id.to_string()),
                      updated_at: Set(Utc::now().naive_utc()),
                      created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                      deleted_at: Set(parse_date_opt(item.deleted_at)),
                  };
                  cash_drawers::Entity::insert(am).on_conflict(OnConflict::column(cash_drawers::Column::Id).update_columns([cash_drawers::Column::Name, cash_drawers::Column::Description, cash_drawers::Column::IsActive, cash_drawers::Column::UpdatedAt, cash_drawers::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                  ids.push(json!({"id": item.id, "server_id": item.id}));
             }
             results.insert("cashDrawers", ids);
        }

        // 19. Shifts
        if let Some(items) = payload.shifts {
             let mut ids = Vec::new();
             for item in items {
                 if let Some(id) = item.id {
                      let status_enum = match item.status.as_str() { "ACTIVE" => ShiftStatus::Active, "CLOSED" => ShiftStatus::Closed, _ => ShiftStatus::Active };
                      let am = shifts::ActiveModel {
                          id: Set(id.clone()),
                          local_ref_id: Set(item.local_ref_id),
                          cash_drawer_id: Set(item.cash_drawer_id),
                          user_id: Set(item.user_id),
                          start_time: Set(NaiveDateTime::parse_from_str(&item.start_time, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap_or_else(|_| Utc::now().naive_utc())),
                          end_time: Set(parse_date_opt(item.end_time)),
                          initial_balance: Set(item.initial_balance),
                          final_balance: Set(item.final_balance),
                          expected_balance: Set(item.expected_balance),
                          difference: Set(item.difference),
                          status: Set(status_enum),
                          note: Set(item.note),
                          organization_id: Set(org_id.to_string()),
                          updated_at: Set(Utc::now().naive_utc()),
                          created_at: Set(parse_date_opt(item.created_at).unwrap_or_else(|| Utc::now().naive_utc())),
                          deleted_at: Set(parse_date_opt(item.deleted_at)),
                      };
                      shifts::Entity::insert(am).on_conflict(OnConflict::column(shifts::Column::Id).update_columns([shifts::Column::EndTime, shifts::Column::FinalBalance, shifts::Column::Status, shifts::Column::UpdatedAt, shifts::Column::DeletedAt]).to_owned()).exec(db).await.ok();
                      ids.push(json!({"id": id, "server_id": id}));
                 }
             }
             results.insert("shifts", ids);
        }

        Ok(json!(results))
    }
}
