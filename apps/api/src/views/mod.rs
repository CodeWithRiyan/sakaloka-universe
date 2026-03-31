//! Response DTOs and view helpers for the Sakaloka API.
//!
//! Each sub-module contains the serializable response types for a specific
//! domain entity.  Shared envelope types (`ApiResponse`, `PaginatedResponse`,
//! `PageMeta`) live in this root module so every controller can reuse them.

pub mod auth;
pub mod brand;
pub mod category;
pub mod order;
pub mod organization;
pub mod product;
pub mod role;
pub mod stock;
pub mod user;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Produce a URL-friendly slug from a name.
///
/// Converts to lowercase, replaces non-alphanumeric characters with hyphens,
/// and collapses consecutive hyphens.
pub fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Standard API response envelope matching Venus frontend expectations.
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    /// Whether the request succeeded.
    pub success: bool,
    /// Human-readable status message.
    pub message: String,
    /// The response payload (omitted on errors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// A single error string (omitted on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Multiple validation error strings (omitted when not applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Pagination metadata matching Venus `IPageMeta`.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    /// Current page number (1-indexed).
    pub page: u64,
    /// Items per page.
    pub limit: u64,
    /// Total number of matching records.
    pub total: u64,
    /// Total number of pages.
    pub pages: u64,
    /// Whether a next page exists.
    pub has_next_page: bool,
    /// Whether a previous page exists.
    pub has_prev_page: bool,
}

/// Inner data envelope for paginated list responses.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedData<T: Serialize> {
    /// The page of results.
    pub data: Vec<T>,
    /// Pagination metadata.
    pub pagination: PageMeta,
    /// Applied sort filters.
    pub filters: ListFilters,
}

/// Active sort filters echoed back to the frontend.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListFilters {
    /// Column used for sorting.
    pub sort_by: String,
    /// Sort direction (`asc` or `desc`).
    pub sort_order: String,
}

/// Paginated list wrapper returned by list endpoints.
///
/// Shape: `{ success, message, data: { data: [...], pagination, filters } }`
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    /// Always `true` for successful list queries.
    pub success: bool,
    /// Human-readable status message.
    pub message: String,
    /// Nested data envelope containing items, pagination, and filters.
    pub data: PaginatedData<T>,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

impl<T: Serialize> ApiResponse<T> {
    /// Build a 200 OK response with data.
    pub fn ok(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            error: None,
            errors: None,
        }
    }

    /// Build a 201 Created response with data.
    pub fn created(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            error: None,
            errors: None,
        }
    }

    /// Build an error response (no data).
    pub fn error(error: &str, message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            error: Some(error.to_string()),
            errors: None,
        }
    }

    /// Build a 404 Not Found response for a named entity.
    pub fn not_found(entity: &str) -> Self {
        Self {
            success: false,
            message: format!("{entity} not found"),
            data: None,
            error: Some("not_found".to_string()),
            errors: None,
        }
    }

    /// Build a validation error response with multiple messages.
    pub fn validation(errors: Vec<String>) -> Self {
        Self {
            success: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some("validation_error".to_string()),
            errors: Some(errors),
        }
    }
}

impl PageMeta {
    /// Compute pagination metadata from the current page, limit, and total
    /// record count.
    pub fn new(page: u64, limit: u64, total: u64) -> Self {
        let pages = if limit == 0 { 0 } else { total.div_ceil(limit) };
        Self {
            page,
            limit,
            total,
            pages,
            has_next_page: page < pages,
            has_prev_page: page > 1,
        }
    }
}

impl ListFilters {
    /// Build filters from pagination query parameters.
    pub fn from_params(params: &PaginationParams) -> Self {
        Self {
            sort_by: params
                .sort_by
                .clone()
                .unwrap_or_else(|| "createdAt".to_string()),
            sort_order: params
                .sort_order
                .clone()
                .unwrap_or_else(|| "desc".to_string()),
        }
    }
}

/// Shared query parameters for paginated list endpoints.
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    /// Page number (1-indexed, default 1).
    pub page: Option<u64>,
    /// Items per page (default 10).
    pub limit: Option<u64>,
    /// Free-text search filter.
    pub search: Option<String>,
    /// Column name to sort by.
    pub sort_by: Option<String>,
    /// Sort direction: `asc` or `desc` (default `asc`).
    pub sort_order: Option<String>,
}

impl PaginationParams {
    /// Resolved page number (minimum 1).
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    /// Resolved page size (minimum 1, maximum 100).
    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(10).clamp(1, 100)
    }

    /// Zero-based offset for paginated queries.
    pub fn offset(&self) -> u64 {
        self.page().saturating_sub(1)
    }

    /// Whether the sort direction is descending.
    pub fn is_desc(&self) -> bool {
        self.sort_order
            .as_deref()
            .map(|s| s.eq_ignore_ascii_case("desc"))
            .unwrap_or(false)
    }
}
