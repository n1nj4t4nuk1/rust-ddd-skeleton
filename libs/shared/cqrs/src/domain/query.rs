use std::any::Any;

pub trait Query: Send + Sync {
    fn get_type(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}
