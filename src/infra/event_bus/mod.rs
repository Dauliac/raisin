use multimap::MultiMap;
use async_trait::async_trait;
use priority_queue::PriorityQueue;
use std::sync::Arc;

use crate::app::cqrs_es::{Events, EventHandlers, EventBus};

pub struct MemoryEventBus {
    listeners: MultiMap<Arc<Events>, EventHandlers>,
    queue: PriorityQueue<Arc<Events>, u8>,
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
        let event = Arc::new(event);
        self.listeners.insert(event, handler);
    }

    async fn publish(&self, event: Events, priority: Option<u8>) {
        let priority =  match priority {
            Some(priority) => priority,
            None => 0,
        };
        self.queue.push(event.clone, priority);
    }

    async fn run(&mut self) {
        loop {
            match self.queue.peek() {
                Some(event) => {
                    let _listeners = match self.listeners.get_vec(&event) {
                        Some(listeners) => listeners,
                        None => { return }
                    };
                    for listener in self.listeners.iter() {
                        match listener {
                            _ => {
                                let listener: Box<EventHandlers> = Box::new(listener);
                                listener.notify(event.clone());
                            },
                        };
                    }
                },
                None => (),
            }
        }
    }
}

// pub struct ParrallelMemoryEventBus {
//     listeners: MultiMap<Arc<Event>, Listeners>,
//     queue: PriorityQueue<Arc<Event>, u8>,
// }

// impl ParrallelMemoryEventBus  {
//     pub fn new() -> Self {
//         Self {
//             listeners: MultiMap::new(),
//             queue: PriorityQueue::new(),
//         }
//     }
// }

// #[async_trait]
// impl EventBus for ParrallelMemoryEventBus {
//     fn subscribe(&mut self, event: Event, listener: Listeners) {
//         let event = Arc::new(event);
//         self.listeners.insert(event, listener);
//     }

//     async fn publish(&self, event: Event, priority: Option<u8>) {
//         let priority =  match priority {
//             Some(priority) => priority,
//             None => 0,
//         };
//         self.queue.push(event);
//         let _listeners = match self.listeners.get_vec(&event) {
//             Some(listeners) => listeners,
//             None => { return }
//         };
//         for listener in self.listeners.iter() {
//             match listener {
//                 _ => {
//                     let listener: Box<Listener> = Box::new(listener);
//                     listener.notify(event.clone());
//                 },
//             };
//         }
//     }
// }
