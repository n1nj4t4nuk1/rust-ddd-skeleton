//! Value Object for the unique key of a config entry.

/// An immutable Value Object wrapping a `String` that uniquely identifies a
/// [`ConfigEntry`](crate::config_entry::domain::entities::config_entry::ConfigEntry).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConfigKey(String);

impl ConfigKey {
    /// Creates a new `ConfigKey` from a raw string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns a reference to the underlying key string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
