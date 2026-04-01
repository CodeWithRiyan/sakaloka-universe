//! Bill of Materials (BOM) domain models.

use serde::{Deserialize, Serialize};

/// Status of a Bill of Materials.
#[derive(Debug, Clone, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum BomStatus {
    /// Draft — not yet active.
    #[default]
    Draft,
    /// Active — currently in use.
    Active,
    /// Archived — no longer used, kept for history.
    Archived,
}

impl std::fmt::Display for BomStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Active => write!(f, "active"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl std::str::FromStr for BomStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            _ => Err(format!("invalid status: {}", s)),
        }
    }
}

/// Bill of Materials — parent assembly linked to a product.
///
/// Represents a recipe (F&B) or material list (Clothing) for a product.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Bom {
    /// Unique identifier.
    pub id: String,
    /// Name of the BOM.
    pub name: String,
    /// Product this BOM belongs to.
    pub product_id: String,
    /// Organization owner.
    pub organization_id: String,
    /// Version string (e.g., "1.0.0").
    pub version: String,
    /// Current status.
    pub status: BomStatus,
    /// When this version becomes effective.
    pub effective_from: Option<chrono::DateTime<chrono::Utc>>,
    /// When this version ends (optional).
    pub effective_to: Option<chrono::DateTime<chrono::Utc>>,
    /// Total cost calculated from items.
    pub total_cost: Option<i64>,
    /// Optional notes.
    pub notes: Option<String>,
    /// User who created this BOM.
    pub created_by: Option<String>,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// BOM item — a component in a BOM.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BomItem {
    /// Unique identifier.
    pub id: String,
    /// Parent BOM ID.
    pub bom_id: String,
    /// Component product ID.
    pub component_product_id: String,
    /// Quantity needed (in specified unit).
    pub quantity: f64,
    /// Unit of measure (g, ml, pcs, meters, etc.).
    pub unit: String,
    /// Expected waste percentage.
    pub waste_percent: f64,
    /// Expected yield percentage.
    pub yield_percent: f64,
    /// Snapshot cost per unit at BOM creation.
    pub unit_cost: Option<i64>,
    /// Calculated line cost (qty × unit_cost).
    pub line_cost: Option<i64>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Status of a production run.
#[derive(Debug, Clone, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProductionRunStatus {
    /// Planned but not yet started.
    #[default]
    Planned,
    /// Currently in progress.
    InProgress,
    /// Completed successfully.
    Completed,
    /// Cancelled before completion.
    Cancelled,
}

impl std::fmt::Display for ProductionRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planned => write!(f, "planned"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::str::FromStr for ProductionRunStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "planned" => Ok(Self::Planned),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("invalid status: {}", s)),
        }
    }
}

/// Production run — tracks actual production of a BOM.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProductionRun {
    /// Unique identifier.
    pub id: String,
    /// BOM being produced.
    pub bom_id: String,
    /// Organization owner.
    pub organization_id: String,
    /// Quantity produced in this run.
    pub quantity_produced: f64,
    /// Estimated cost based on BOM.
    pub estimated_cost: Option<i64>,
    /// Actual cost after completion.
    pub actual_cost: Option<i64>,
    /// Current status.
    pub status: ProductionRunStatus,
    /// When production started.
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When production completed.
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// User who created this run.
    pub created_by: Option<String>,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// BOM consumption — actual consumption per production run.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Consumption {
    /// Unique identifier.
    pub id: String,
    /// Production run this consumption belongs to.
    pub production_run_id: String,
    /// Component product consumed.
    pub component_product_id: String,
    /// Planned quantity (from BOM).
    pub planned_quantity: f64,
    /// Actual quantity consumed.
    pub actual_quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Actual waste quantity.
    pub waste_quantity: f64,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
}
