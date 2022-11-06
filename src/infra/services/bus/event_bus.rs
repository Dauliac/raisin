use multimap::MultiMap;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use priority_queue::PriorityQueue;

use crate::{app::cqrs_es::event::{
    Events,
    EventHandlers,
    EventBus,
}, core::domain::Event};

pub struct MemoryEventBus {
    listeners: Arc<RwLock<MultiMap<String, EventHandlers>>>,
    queue: Arc<RwLock<PriorityQueue<Events, u8>>>,
}

impl MemoryEventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(MultiMap::new())),
            queue: Arc::new(RwLock::new(PriorityQueue::new())),
        }
    }
}

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn subscribe(&mut self, event: Events, handler: EventHandlers) {
        self.listeners.write().await.insert(event.get_id(), handler);
    }

    async fn publish(&mut self, event: Events) {
        let priority = match event {
            Events::Domain(ref event) => event.priority.clone(),
            Events::DomainError(ref event) => event.priority.clone(),
        };
        self.queue.write().await.push(event, priority as u8);
    }

    async fn run(&mut self) {
        let queue = self.queue.clone();
        let listeners = self.listeners.clone();

        // Spawn thread to notify about events
        tokio::spawn(async move {
            loop {
                match queue.write().await.peek() {
                    Some((event, _priority)) => {
                        todo!("change multimap by a tree ?");
                        let id = &event.get_id();
                        let handlers_for_current_event = match listeners.read().await.get_vec(id) {
                            Some(listeners) => listeners,
                            None => { return }
                        };
                        for handler in handlers_for_current_event.iter_mut() {
                            match handler {
                                EventHandlers::Logger(logger) => {
                                    let logger = logger.write().await;
                                    logger.notify(event.clone());
                                },
                            };

                        }
                    },
                    None => (),
                }
            }
        });
    }
}
