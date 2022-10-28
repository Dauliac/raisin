use std::{time::SystemTime, sync::{Arc, RwLock}};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    core::domain::Aggregate,
    domain::program::Program,
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

#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Hash, Clone)]
pub struct Event<E> {
    pub event: E,
    pub timestamp: SystemTime,
    pub priority: EventPriority,
}

impl<E> Event<E> {
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Events {
    Domain(Event<DomainEvent>),
    DomainError(Event<DomainError>),
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
    fn subscribe(&mut self, event: Events, priority: EventPriority, handler: EventHandlers);
    async fn publish(&mut self, event: Events);
    async fn run(&mut self);
}
