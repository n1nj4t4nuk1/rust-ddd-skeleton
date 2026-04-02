use config::config_entry::domain::entities::config_entry::ConfigEntry;

use crate::src::config_entry::domain::value_objects::mothers::config_key_mother::ConfigKeyMother;
use crate::src::config_entry::domain::value_objects::mothers::config_value_mother::ConfigValueMother;

pub struct ConfigEntryMother;

#[allow(dead_code)]
impl ConfigEntryMother {
    pub fn random() -> ConfigEntry {
        ConfigEntry::new(ConfigKeyMother::random(), ConfigValueMother::random())
    }

    pub fn create(key: impl Into<String>, value: impl Into<String>) -> ConfigEntry {
        ConfigEntry::new(
            ConfigKeyMother::create(key),
            ConfigValueMother::create(value),
        )
    }
}
