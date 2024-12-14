use std::sync::Arc;

use async_trait::async_trait;

use super::domain_event::DomainEvent;

#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn handle<'a>(&self, event: Arc<dyn DomainEvent + 'a>);
    fn get_subscriptions(&self) -> Vec<String>;
}
