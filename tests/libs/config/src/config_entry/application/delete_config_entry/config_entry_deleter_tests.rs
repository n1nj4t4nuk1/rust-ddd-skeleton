use std::sync::Arc;

use config::config_entry::application::delete_config_entry::config_entry_deleter::ConfigEntryDeleter;
use config::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use config::config_entry::domain::events::config_entry_deleted_event::ConfigEntryDeletedEvent;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::config_entry::domain::entities::mothers::config_entry_mother::ConfigEntryMother;
use crate::src::config_entry::domain::value_objects::mothers::config_key_mother::ConfigKeyMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::config_entry_repository_mock::ConfigEntryRepositoryMock;

fn make_deleter(
    repo: Arc<ConfigEntryRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> ConfigEntryDeleter {
    let repo: Arc<dyn ConfigEntryRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    ConfigEntryDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let entry = ConfigEntryMother::random();
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry(entry));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(ConfigKeyMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let entry = ConfigEntryMother::random();
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry(entry));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(ConfigKeyMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![ConfigEntryDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_entry_does_not_exist() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(ConfigKeyMother::random()).await;

    assert!(matches!(
        result,
        Err(ConfigEntryRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_entry_not_found() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(ConfigKeyMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
