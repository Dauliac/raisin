use std::any::Any;
use uuid::Uuid;

pub trait Entity {
    fn get_uuid(&self) -> Uuid;
    fn equals(&self, entity: Box<dyn Entity>) -> bool;
}

pub trait Value {
    fn equals(&self, value: &dyn Any) -> bool;
}
