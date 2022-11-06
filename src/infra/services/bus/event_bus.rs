use multimap::MultiMap;
use async_trait::async_trait;
use priority_queue::PriorityQueue;
use strum::VariantNames;

use crate::{app::cqrs_es::event::{
    Events,
    EventHandlers,
    EventBus,
}, core::domain::Event};

pub struct MemoryEventBus {
    listeners: MultiMap<String, EventHandlers>,
    queue: PriorityQueue<Events, u8>,
}

impl MemoryEventBus {
    pub fn new() -> Self {
        Self {
            listeners: MultiMap::new(),
            queue: PriorityQueue::new(),
        }
    }
}

#[async_trait]
impl EventBus for MemoryEventBus {
    fn subscribe(&mut self, event: Events, handler: EventHandlers) {
        self.listeners.insert(event.get_id(), handler);
    }

    async fn publish(&mut self, event: Events) {
        let priority = match event {
            Events::Domain(ref event) => event.priority.clone(),
            Events::DomainError(ref event) => event.priority.clone(),
        };
        self.queue.push(event, priority as u8);
    }

    async fn run(&mut self) {
        loop {
            match self.queue.peek() {
                Some((event, _priority)) => {
                    todo!("change multimap by a tree ?");
                    let id = &event.get_id();
                    let handlers_for_current_event = match self.listeners.get_vec(id) {
                        Some(listeners) => listeners,
                        None => { return }
                    };
                    for handler in handlers_for_current_event.iter_mut() {
                        match handler {
                            EventHandlers::Logger(logger) => {
                                match logger.write() {
                                    Ok(logger) => logger.notify(event.clone()),
                                    Err(error) => {
                                        todo!("Add pannic into logger to stop and log error");
                                        panic!("Failed to lock logger");
                                    }
                                };
                            },
                        };

                    }
                },
                None => (),
            }
        }
    }
}
