pub mod command;

use std::time::SystemTime;

use async_trait::async_trait;

// TODO: have a timestamp in
use crate::{core::domain::Aggregate, domain::program::Program};

type DomainEvent = <Program as Aggregate<Program>>::Event;
type DomainError = <Program as Aggregate<Program>>::Error;
type DomainCommand = <Program as Aggregate<Program>>::Command;

pub enum EventPriority {
    Low = 30,
    Normal = 60,
    High = 90,
}

pub struct Event<E> {
    event: E,
    timestamp: SystemTime,
    priority: EventPriority,
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

pub enum Events {
    Domain(Event<DomainEvent>),
    DomainError(Event<DomainError>),
}

pub trait EventHandler {
    fn notify(event: Events);
}

pub enum EventHandlers {}

#[async_trait]
pub trait EventBus {
    fn subscribe(&mut self, event: Events, priority: u8, handler: EventHandlers);
    async fn publish(&self, event: Events);
    async fn run(&mut self);
}

pub trait Command {
    fn run(&mut self);
}

pub trait Query<R> {
    fn run(&mut self) -> R;
}

// pub struct CommandHandler<Repository> {
//     repository: Box<Repository>,
//     event_bus: Box<dyn EventBus>,
// }
