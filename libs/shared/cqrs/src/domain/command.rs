use std::any::Any;

pub trait Command: Send + Sync {
    fn command_type(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}
