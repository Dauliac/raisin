use multimap::MultiMap;
use async_trait::async_trait;
use priority_queue::PriorityQueue;

use crate::app::cqrs_es::event::{
    Events,
    EventHandlers,
    EventBus,
    EventPriority
};

pub struct MemoryEventBus {
    listeners: MultiMap<Events, EventHandlers>,
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
    fn subscribe(&mut self, event: Events, priority: EventPriority, handler: EventHandlers) {
        self.listeners.insert(event, handler);
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
                Some((event, priority)) => {
                    let _listeners = match self.listeners.get_vec(&event) {
                        Some(listeners) => listeners,
                        None => { return }
                    };
                    for (_, handler) in self.listeners.iter_mut() {
                        match handler {
                            EventHandlers::Logger(logger) => {
                                let mut logger = match logger.write() {
                                    Ok(logger) => logger,
                                    Err(error) => {
                                        todo!("Add pannic into logger to stop and log error");
                                        panic!("Failed to lock logger");
                                    }
                                };
                                logger.notify(event.clone());
                            },
                        };
                    }
                },
                None => (),
            }
        }
    }
}
