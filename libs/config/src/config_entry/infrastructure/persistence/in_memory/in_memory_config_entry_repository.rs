//! In-memory implementation of [`ConfigEntryRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// An in-memory implementation of [`ConfigEntryRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Intended for use in tests and local development.
pub struct InMemoryConfigEntryRepository {
    store: Mutex<HashMap<String, ConfigEntry>>,
}

impl InMemoryConfigEntryRepository {
    /// Creates a new empty `InMemoryConfigEntryRepository`.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryConfigEntryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConfigEntryRepository for InMemoryConfigEntryRepository {
    async fn save(&self, entry: &ConfigEntry) -> Result<(), ConfigEntryRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let key = entry.key().value().to_string();
        if store.contains_key(&key) {
            return Err(ConfigEntryRepositoryError::AlreadyExists);
        }
        store.insert(key, entry.clone());
        Ok(())
    }

    async fn find_by_key(
        &self,
        key: &ConfigKey,
    ) -> Result<Option<ConfigEntry>, ConfigEntryRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(key.value()).cloned())
    }

    async fn update(&self, entry: &ConfigEntry) -> Result<(), ConfigEntryRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let key = entry.key().value().to_string();
        if !store.contains_key(&key) {
            return Err(ConfigEntryRepositoryError::NotFound);
        }
        store.insert(key, entry.clone());
        Ok(())
    }

    async fn delete(&self, key: &ConfigKey) -> Result<(), ConfigEntryRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(key.value()).is_none() {
            return Err(ConfigEntryRepositoryError::NotFound);
        }
        Ok(())
    }
}
