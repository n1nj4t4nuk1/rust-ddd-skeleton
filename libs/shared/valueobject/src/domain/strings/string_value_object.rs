//! Value Object wrapper for arbitrary string values.

/// An immutable Value Object wrapping a [`String`].
///
/// Accepts any non-validated string. Use domain-specific subtypes when
/// additional constraints are needed.
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::strings::string_value_object::StringValueObject;
///
/// let name = StringValueObject::new("Alice".to_string());
/// assert_eq!(name.value(), "Alice");
/// ```
#[derive(Clone)]
pub struct StringValueObject(String);

impl StringValueObject {
    /// Creates a new `StringValueObject` with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The underlying string value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the underlying string slice.
    pub fn value(&self) -> &str {
        &self.0
    }
}
