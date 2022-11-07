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
    handlers: MultiMap<Events, EventHandlers>,
    queue: Arc<RwLock<PriorityQueue<Events, u8>>>,
}

impl MemoryEventBus {
    pub fn new() -> Self {
        Self {
            handlers: MultiMap::new(),
            queue: Arc::new(RwLock::new(PriorityQueue::new())),
        }
    }
}

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn subscribe(&mut self, event: Events, handler: EventHandlers) {
        self.handlers.insert(event, handler);
    }

    async fn publish(&mut self, event: Events) {
        let priority = match event {
            Events::Domain(ref event) => event.priority.clone(),
            Events::DomainError(ref event) => event.priority.clone(),
        };

        let mut write_queue = self.queue.write().await;
        write_queue.push(event, priority as u8);
    }

    async fn run(&mut self) {
        let mut queue = self.queue.write().await;
        let list = queue.pop();

        match list {
            Some((event, _priority)) => {
                let id = &event.get_id().clone();
                let handlers: Vec<&EventHandlers> = self.handlers
                    .iter()
                    .filter_map(|(to_check_event, handler)| {
                        if to_check_event.is_child_event(&event) { Some(handler) } else {None} })
                    .collect();

                for handler in handlers.iter() {
                    let event = event.clone();
                    match handler {
                        EventHandlers::Logger(logger) => {
                            let logger = logger.clone();
                            tokio::spawn(async move {
                                let mut logger = logger.write().await;
                                logger.notify(event);
                            });
                        },
                    };
                }
            },
            None => {},
        };
    }
}
