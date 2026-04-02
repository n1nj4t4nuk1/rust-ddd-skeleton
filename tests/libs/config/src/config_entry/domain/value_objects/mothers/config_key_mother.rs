use config::config_entry::domain::value_objects::config_key::ConfigKey;
use uuid::Uuid;

pub struct ConfigKeyMother;

#[allow(dead_code)]
impl ConfigKeyMother {
    pub fn create(value: impl Into<String>) -> ConfigKey {
        ConfigKey::new(value)
    }

    pub fn random() -> ConfigKey {
        ConfigKey::new(Uuid::new_v4().to_string())
    }
}
