use std::{time::SystemTime, sync::{Arc, RwLock}};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};

use crate::{
    core::domain::{Aggregate, Event as EventAbstract},
    domain::{program::Program, sources::aggregate::SourcesEvent},
    app::services::logger::Logger
};

type DomainEvent = <Program as Aggregate<Program>>::Event;
type DomainError = <Program as Aggregate<Program>>::Error;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub enum EventPriority {
    Low = 30,
    Normal = 60,
    High = 90,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Event<E: Default> {
    pub event: E,
    pub timestamp: SystemTime,
    pub priority: EventPriority,
}

impl <E: Default> Default for Event<E> {
    fn default() -> Self {
        Self {
          event: E::default(),
          timestamp: SystemTime::now(),
          priority: EventPriority::Normal
        }
    }
}

impl<E: Default> Event<E> {
    fn new(event: E, priority: Option<EventPriority>) -> Self {
        let priority = match priority {
            Some(priority) => priority,
            None => EventPriority::Normal,
        };
        Self {
            event,
            timestamp: SystemTime::now(),
            priority,
        }
    }
}

#[derive(Debug, EnumString, EnumVariantNames, IntoStaticStr, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[strum(serialize_all = "kebab_case")]
pub enum Events {
    Domain(Event<DomainEvent>),
    DomainError(Event<DomainError>),
}
impl Default for Events {
    fn default() -> Self {
        Self::Domain(Event::default())
    }
}
impl EventAbstract<Events> for Events {
    fn get_id(&self) -> String {
        match &self {
            Events::Domain(app_event) => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, app_event.event.get_id().as_str())
            },
            Events::DomainError(app_event) => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, app_event.event.get_id().as_str())
            }
        }
    }
}

impl Events {
    pub fn new_domain(event: DomainEvent) -> Self {
        let event = Event::new(event, Some(EventPriority::High));
        Events::Domain(event)
    }
    pub fn new_domain_error(event: DomainError) -> Self {
        let event = Event::new(event, Some(EventPriority::High));
        Events::DomainError(event)
    }
}

pub trait EventHandler: Send + Sync {
    fn notify(&self, event: Events);
}

pub enum EventHandlers {
    Logger(Arc<RwLock<dyn Logger + Send + Sync>>)
}

#[async_trait]
pub trait EventBus {
    fn subscribe(&mut self, event: Events, handler: EventHandlers);
    async fn publish(&mut self, event: Events);
    async fn run(&mut self);
}
