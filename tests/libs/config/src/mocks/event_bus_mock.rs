use std::sync::Mutex;

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::domain::event_bus_error::EventBusError;

pub struct EventBusMock {
    should_fail: bool,
    published_event_names: Mutex<Vec<&'static str>>,
}

#[allow(dead_code)]
impl EventBusMock {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            published_event_names: Mutex::new(vec![]),
        }
    }

    pub fn that_fails() -> Self {
        Self {
            should_fail: true,
            published_event_names: Mutex::new(vec![]),
        }
    }

    pub fn published_event_names(&self) -> Vec<&'static str> {
        self.published_event_names.lock().unwrap().clone()
    }
}

impl EventBus for EventBusMock {
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError> {
        if self.should_fail {
            return Err(EventBusError::DispatchError("mock error".to_string()));
        }
        let mut names = self.published_event_names.lock().unwrap();
        for event in &events {
            names.push(event.event_name());
        }
        Ok(())
    }
}
