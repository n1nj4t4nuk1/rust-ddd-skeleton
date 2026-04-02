use std::sync::Mutex;

use async_trait::async_trait;

use config::config_entry::domain::entities::config_entry::ConfigEntry;
use config::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use config::config_entry::domain::value_objects::config_key::ConfigKey;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByKeyBehavior {
    ReturnsNone,
    ReturnsEntry(Mutex<Option<ConfigEntry>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct ConfigEntryRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_key_behavior: FindByKeyBehavior,
    update_behavior: UpdateBehavior,
    saved_keys: Mutex<Vec<String>>,
    update_call_count: Mutex<u32>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl ConfigEntryRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_key_behavior: FindByKeyBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            find_by_key_behavior: FindByKeyBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_entry(entry: ConfigEntry) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_key_behavior: FindByKeyBehavior::ReturnsEntry(Mutex::new(Some(entry))),
            update_behavior: UpdateBehavior::Succeeds,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_entry_but_update_fails(entry: ConfigEntry) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_key_behavior: FindByKeyBehavior::ReturnsEntry(Mutex::new(Some(entry))),
            update_behavior: UpdateBehavior::FailsWithNotFound,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_key_behavior: FindByKeyBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_key_behavior: FindByKeyBehavior::FailsWithUnexpected(message),
            update_behavior: UpdateBehavior::Succeeds,
            saved_keys: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_keys(&self) -> Vec<String> {
        self.saved_keys.lock().unwrap().clone()
    }

    pub fn update_call_count(&self) -> u32 {
        *self.update_call_count.lock().unwrap()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl ConfigEntryRepository for ConfigEntryRepositoryMock {
    async fn save(&self, entry: &ConfigEntry) -> Result<(), ConfigEntryRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => {
                Err(ConfigEntryRepositoryError::AlreadyExists)
            }
            SaveBehavior::Succeeds => {
                self.saved_keys
                    .lock()
                    .unwrap()
                    .push(entry.key().value().to_string());
                Ok(())
            }
        }
    }

    async fn find_by_key(
        &self,
        _key: &ConfigKey,
    ) -> Result<Option<ConfigEntry>, ConfigEntryRepositoryError> {
        match &self.find_by_key_behavior {
            FindByKeyBehavior::ReturnsNone => Ok(None),
            FindByKeyBehavior::ReturnsEntry(cell) => Ok(cell.lock().unwrap().take()),
            FindByKeyBehavior::FailsWithUnexpected(msg) => {
                Err(ConfigEntryRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn update(
        &self,
        _entry: &ConfigEntry,
    ) -> Result<(), ConfigEntryRepositoryError> {
        *self.update_call_count.lock().unwrap() += 1;
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithNotFound => Err(ConfigEntryRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(ConfigEntryRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _key: &ConfigKey) -> Result<(), ConfigEntryRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        Ok(())
    }
}
