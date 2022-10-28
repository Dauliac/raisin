use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use priority_queue::PriorityQueue;

use crate::{domain::repository::Repository, app::cqrs_es::{cqrs::{CommandBus, Commands}, event::Events}};

pub struct MemoryCommandBus {
    queue: PriorityQueue<Commands, u8>,
    repository: Arc<RwLock<dyn Repository>>,
}

impl MemoryCommandBus {
    pub fn new(repository: Arc<RwLock<dyn Repository>>) -> Self {
        Self {
            repository,
            queue: PriorityQueue::new(),
        }
    }
}

#[async_trait]
impl CommandBus for MemoryCommandBus {
    async fn publish(&mut self, command: Commands) {
        let priority = match command {
            Commands::Domain(ref command) => command.priority.clone(),
        };

        self.queue.push(command, priority as u8);
    }
    async fn run(&mut self) {
        loop {
            match self.queue.peek() {
                Some((command, _priority)) => {
                    let command = command.to_owned();
                    match command {
                        Commands::Domain(mut command) => {
                            command.run(self.repository.clone());
                        },
                    }
                },
                None => (),
            };
        }
    }
}
