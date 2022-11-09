use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum::VariantNames;
use std::{fmt::Debug, hash::Hash};
use uuid::Uuid as ExternalUuid;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Uuid(ExternalUuid);

pub fn new_uuid() -> Uuid {
    Uuid(ExternalUuid::new_v4())
}

pub trait Entity<T> {
    type Uuid;

    fn uuid(&self) -> Self::Uuid;
    fn equals(&self, entity: Box<T>) -> bool;
}

pub trait Value<T>: Default + Debug + Clone + Serialize + DeserializeOwned + PartialEq + Hash + Eq {
    fn equals(&self, value: &T) -> bool;
}

pub trait Const: Default + Debug + Clone + Serialize + DeserializeOwned + PartialEq + Hash + Eq {}


pub trait Event<T>: Debug + Clone + VariantNames + Into<&'static str> + Serialize + DeserializeOwned + PartialEq + Hash + Eq + {
    const SEPARATOR: &'static str = "::";
    fn id_variants() -> Vec<String> {
        let mut variants = vec![];
        for id in Self::VARIANTS {
            variants.push(format!("{}{}", Self::SEPARATOR, id));
        }
        variants
    }

    fn id(&self) -> String {
        let id: &'static str = self.clone().into();
        format!("{}{}", Self::SEPARATOR, id)
    }

    fn is_child_event(&self, event: &Self) -> bool {
        let id = event.id();
        let is_child = id.starts_with(&self.id());
        is_child
    }
}

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
