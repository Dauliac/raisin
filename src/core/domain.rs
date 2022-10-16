use serde::{de::DeserializeOwned, Serialize};
use std::{any::Any, hash::Hash};
use async_trait::async_trait;

pub use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

pub trait Entity<T> {
    type Uuid;

    fn get_uuid(&self) -> Self::Uuid;
    fn equals(&self, entity: Box<T>) -> bool;
}

pub trait Value: Clone + PartialEq {
    fn equals(&self, value: &dyn Any) -> bool;
}

pub trait Event: Hash + Eq {}

#[async_trait]
pub trait Aggregate<T>: Entity<T> + Serialize + DeserializeOwned + Sync + Send {
    type Error;
    type Event;
    type Command;
    // type Result = Result<Vec<Self::Event>, Self::Error>;
    type Result;
    async fn handle(&self, command: Self::Command) -> Self::Result;
    fn apply(&mut self, event: Self::Event);
}
