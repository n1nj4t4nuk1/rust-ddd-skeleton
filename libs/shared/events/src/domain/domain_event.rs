pub trait DomainEvent: Sync + Send {
    fn event_type(&self) -> String;
}
