use std::sync::Arc;

use config::config_entry::application::update_config_entry::config_entry_updater::ConfigEntryUpdater;
use config::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use config::config_entry::domain::events::config_entry_updated_event::ConfigEntryUpdatedEvent;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::config_entry::domain::entities::mothers::config_entry_mother::ConfigEntryMother;
use crate::src::config_entry::domain::value_objects::mothers::config_key_mother::ConfigKeyMother;
use crate::src::config_entry::domain::value_objects::mothers::config_value_mother::ConfigValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::config_entry_repository_mock::ConfigEntryRepositoryMock;

fn make_updater(
    repo: Arc<ConfigEntryRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> ConfigEntryUpdater {
    let repo: Arc<dyn ConfigEntryRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    ConfigEntryUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let entry = ConfigEntryMother::random();
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry(entry));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await
        .unwrap();

    assert_eq!(repo.update_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let entry = ConfigEntryMother::random();
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry(entry));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![ConfigEntryUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_entry_does_not_exist() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await;

    assert!(matches!(
        result,
        Err(ConfigEntryRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let entry = ConfigEntryMother::random();
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry_but_update_fails(entry));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(ConfigKeyMother::random(), ConfigValueMother::random())
        .await;

    assert!(bus.published_event_names().is_empty());
}
