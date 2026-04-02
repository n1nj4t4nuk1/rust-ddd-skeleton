use std::sync::Arc;

use config::config_entry::application::create_config_entry::config_entry_creator::ConfigEntryCreator;
use config::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use config::config_entry::domain::events::config_entry_created_event::ConfigEntryCreatedEvent;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::config_entry::domain::value_objects::mothers::config_key_mother::ConfigKeyMother;
use crate::src::config_entry::domain::value_objects::mothers::config_value_mother::ConfigValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::config_entry_repository_mock::ConfigEntryRepositoryMock;

fn make_creator(
    repo: Arc<ConfigEntryRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> ConfigEntryCreator {
    let repo: Arc<dyn ConfigEntryRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    ConfigEntryCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_entry() {
    let key = ConfigKeyMother::random();
    let expected_key = key.value().to_string();

    let repo = Arc::new(ConfigEntryRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(key, ConfigValueMother::random())
        .await
        .unwrap();

    assert_eq!(repo.saved_keys(), vec![expected_key]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![ConfigEntryCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_ok() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn it_returns_already_exists_error_when_entry_already_exists() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await;

    assert!(matches!(
        result,
        Err(ConfigEntryRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await;

    assert!(bus.published_event_names().is_empty());
}
