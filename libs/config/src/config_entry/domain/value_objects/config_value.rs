//! Value Object for the value stored in a config entry.

/// An immutable Value Object wrapping a `String` that holds the content
/// of a [`ConfigEntry`](crate::config_entry::domain::entities::config_entry::ConfigEntry).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigValue(String);

impl ConfigValue {
    /// Creates a new `ConfigValue` from a raw string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns a reference to the underlying value string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
