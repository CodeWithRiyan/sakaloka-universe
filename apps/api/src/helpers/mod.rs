//! Shared helpers used across multiple controllers.
//!
//! This module is the API-layer home for cross-controller utilities.
//! Domain-specific logic should remain inside its controller module or move to
//! a shared library crate when it becomes business-domain behavior.

pub mod error_map;
pub mod org_resolver;
pub mod patch_builder;
