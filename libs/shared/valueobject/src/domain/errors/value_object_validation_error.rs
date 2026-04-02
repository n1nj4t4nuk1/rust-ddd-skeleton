//! Validation error for Value Objects.

use std::fmt;

/// Error returned when a Value Object fails its domain invariant checks.
///
/// Wraps a human-readable message that describes which constraint was violated.
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
///
/// let err = ValueObjectValidationError::new("-5 is not a positive integer".to_string());
/// assert!(err.to_string().contains("ValueObject validation error"));
/// ```
#[derive(Debug)]
pub struct ValueObjectValidationError(String);

impl ValueObjectValidationError {
    /// Creates a new `ValueObjectValidationError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A description of the violated domain invariant.
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Display for ValueObjectValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValueObject validation error: {}", self.0)
    }
}

impl std::error::Error for ValueObjectValidationError {}
