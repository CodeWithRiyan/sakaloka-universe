//! Shared traits for Sakaloka-Universe domain objects.

use std::fmt;

/// Trait implemented by all domain entities that carry a unique identifier.
///
/// # Examples
///
/// ```rust
/// use sakaloka_core::traits::Identifiable;
///
/// struct MyEntity { id: String }
///
/// impl Identifiable for MyEntity {
///     fn id(&self) -> &str { &self.id }
/// }
///
/// let e = MyEntity { id: "entity:01".to_string() };
/// assert_eq!(e.id(), "entity:01");
/// ```
pub trait Identifiable {
    /// Returns the unique identifier of this entity.
    fn id(&self) -> &str;
}

/// Trait for types that can validate their own internal state.
///
/// Used by newtypes to verify input on construction before the value
/// is admitted into business logic.
pub trait Validate: Sized {
    /// The error type returned when validation fails.
    type Error: fmt::Display;

    /// Validates the value, returning `Ok(Self)` if valid.
    ///
    /// # Errors
    ///
    /// Returns `Err(Self::Error)` when the value does not satisfy
    /// the validation rules for this type.
    fn validate(self) -> Result<Self, Self::Error>;
}
