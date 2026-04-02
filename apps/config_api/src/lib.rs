use std::sync::Arc;

use actix_web::web;
use config::config_entry::application::create_config_entry::config_entry_creator::ConfigEntryCreator;
use config::config_entry::application::create_config_entry::create_config_entry_command_handler::CreateConfigEntryCommandHandler;
use config::config_entry::application::delete_config_entry::config_entry_deleter::ConfigEntryDeleter;
use config::config_entry::application::delete_config_entry::delete_config_entry_command_handler::DeleteConfigEntryCommandHandler;
use config::config_entry::application::find_config_entry::config_entry_finder::ConfigEntryFinder;
use config::config_entry::application::find_config_entry::find_config_entry_query_handler::FindConfigEntryQueryHandler;
use config::config_entry::application::update_config_entry::config_entry_updater::ConfigEntryUpdater;
use config::config_entry::application::update_config_entry::update_config_entry_command_handler::UpdateConfigEntryCommandHandler;
use config::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use config::config_entry::infrastructure::persistence::in_memory::in_memory_config_entry_repository::InMemoryConfigEntryRepository;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

pub mod config_entry;
pub mod health;

/// Shared application state injected into every Actix-Web request handler.
pub struct AppState {
    pub command_bus: Arc<dyn CommandBus>,
    pub query_bus: Arc<dyn QueryBus>,
}

/// Wires all repositories, services and buses together and returns the shared
/// application state. Each call produces an isolated in-memory store, making
/// this function safe to call once per test.
pub fn build_state() -> web::Data<AppState> {
    let repo: Arc<dyn ConfigEntryRepository> = Arc::new(InMemoryConfigEntryRepository::new());
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    let creator = ConfigEntryCreator::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let create_handler = CreateConfigEntryCommandHandler::new(creator);

    let finder = ConfigEntryFinder::new(Arc::clone(&repo));
    let find_handler = FindConfigEntryQueryHandler::new(finder);

    let updater = ConfigEntryUpdater::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let update_handler = UpdateConfigEntryCommandHandler::new(updater);

    let deleter = ConfigEntryDeleter::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let delete_handler = DeleteConfigEntryCommandHandler::new(deleter);

    let mut command_bus = InMemoryCommandBus::new();
    command_bus
        .register(create_handler)
        .expect("Failed to register CreateConfigEntryCommandHandler");
    command_bus
        .register(update_handler)
        .expect("Failed to register UpdateConfigEntryCommandHandler");
    command_bus
        .register(delete_handler)
        .expect("Failed to register DeleteConfigEntryCommandHandler");
    let command_bus: Arc<dyn CommandBus> = Arc::new(command_bus);

    let mut query_bus = InMemoryQueryBus::new();
    query_bus
        .register(find_handler)
        .expect("Failed to register FindConfigEntryQueryHandler");
    let query_bus: Arc<dyn QueryBus> = Arc::new(query_bus);

    web::Data::new(AppState { command_bus, query_bus })
}

/// Registers all HTTP routes onto an Actix-Web [`ServiceConfig`].
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get::handler)
        .service(config_entry::post::handler)
        .service(config_entry::get::handler)
        .service(config_entry::put::handler)
        .service(config_entry::delete::handler);
}
