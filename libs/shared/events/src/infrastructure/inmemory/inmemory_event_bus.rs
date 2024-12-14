use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::domain::{domain_event::DomainEvent, event_bus::EventBus, event_handler::EventHandler};

pub struct InMemoryEventBus {
    pub handlers_by_sub: HashMap<String, Vec<Arc<dyn EventHandler>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        InMemoryEventBus {
            handlers_by_sub: HashMap::new(),
        }
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish<'a>(&self, event: Arc<dyn DomainEvent + 'a>) {
        let event_type = event.event_type();
        if let Some(handlers) = self.handlers_by_sub.get(&event_type) {
            for handler in handlers {
                handler.handle(event.clone()).await;
            }
        }
    }
}

pub struct MyEvent {}

impl DomainEvent for MyEvent {
    fn event_type(&self) -> String {
        "MyEvent".to_string()
    }
}
