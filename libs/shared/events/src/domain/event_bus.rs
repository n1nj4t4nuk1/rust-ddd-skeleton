use std::sync::Arc;

use async_trait::async_trait;

use super::domain_event::DomainEvent;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {
    async fn publish<'a>(&self, event: Arc<dyn DomainEvent + 'a>);
}
