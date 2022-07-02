use multimap::MultiMap;
use async_trait::async_trait;
use priority_queue::PriorityQueue;
use std::sync::Arc;

use crate::domain::{sources::aggregate::SourcesEvent, cfg::aggregate::CfgEvent};

pub enum Listener {
}

#[derive(PartialEq, Eq, Hash)]
pub enum Event {
    Sources(SourcesEvent),
    Cfg(CfgEvent),
}

#[async_trait]
pub trait EventBus<Event, Listener> {
    fn subscribe(&mut self, event: Event, priority: u8, listeners: Listener);
    async fn publish(&self, event: Event);
}

pub struct MemoryEventBus {
    listeners: MultiMap<Arc<Event>, Listener>,
    queue: PriorityQueue<Arc<Event>, u8>,
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
impl EventBus<Event, Listener> for MemoryEventBus {
    fn subscribe(&mut self, event: Event, priority: u8, listener: Listener) {
        let event = Arc::new(event);
        self.queue.push(event.clone(), priority);
        self.listeners.insert(event, listener);
    }

    async fn publish(&self, event: Event) {
        let _listeners = match self.listeners.get_vec(&event) {
            Some(listeners) => listeners,
            None => { return }
        };
        // for listener in listeners.iter() {
            // listener
        // }
    }
}
