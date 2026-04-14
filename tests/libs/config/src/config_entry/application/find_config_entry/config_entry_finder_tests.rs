use std::sync::Arc;

use config::config_entry::application::find_config_entry::config_entry_finder::ConfigEntryFinder;
use config::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;

use crate::src::config_entry::domain::entities::mothers::config_entry_mother::ConfigEntryMother;
use crate::src::config_entry::domain::value_objects::mothers::config_key_mother::ConfigKeyMother;
use crate::src::mocks::config_entry_repository_mock::ConfigEntryRepositoryMock;

fn make_finder(repo: Arc<ConfigEntryRepositoryMock>) -> ConfigEntryFinder {
    let repo: Arc<dyn ConfigEntryRepository> = repo;
    ConfigEntryFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_entry_does_not_exist() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(ConfigKeyMother::random()).await;

    assert!(matches!(
        result,
        Err(ConfigEntryRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_returns_entry_when_it_exists() {
    let entry = ConfigEntryMother::create("my-key", "my-value");
    let repo = Arc::new(ConfigEntryRepositoryMock::that_returns_entry(entry));
    let finder = make_finder(repo);

    let result = finder
        .execute(ConfigKeyMother::create("my-key"))
        .await
        .unwrap();

    assert_eq!(result.key().value(), "my-key");
    assert_eq!(result.value().value(), "my-value");
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(ConfigEntryRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(ConfigKeyMother::random()).await;

    assert!(matches!(
        result,
        Err(ConfigEntryRepositoryError::Unexpected(_))
    ));
}
