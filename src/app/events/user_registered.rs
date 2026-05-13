use crate::core::event::traits::Event;
use std::any::Any;

#[derive(Debug)]
pub struct UserRegistered {
    pub name: String,
    pub email: String,
}

impl Event for UserRegistered {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
