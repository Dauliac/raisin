use serde::{de::DeserializeOwned, Serialize};
use std::{any::Any, hash::Hash};

pub use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

pub trait Entity {
    fn get_uuid(&self) -> Uuid;
    fn equals(&self, entity: Box<dyn Entity>) -> bool;
}

pub trait Value: Clone + PartialEq {
    fn equals(&self, value: &dyn Any) -> bool;
}

pub trait Event: Hash + Eq {}

pub trait Aggregate: Entity + Serialize + DeserializeOwned + Sync + Send {
    type Error;
    type Event;
    type Result;
}
