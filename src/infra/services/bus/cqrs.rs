use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use priority_queue::PriorityQueue;

use crate::{domain::repository::Repository, app::cqrs_es::{cqrs::{CommandBus, Commands}, event::{Events, EventBus}}};

pub struct MemoryCommandBus {
    queue: Arc<RwLock<PriorityQueue<Commands, u8>>>,
    repository: Arc<RwLock<dyn Repository>>,
    event_bus: Arc<RwLock<dyn EventBus>>,
}

impl MemoryCommandBus {
    pub fn new(
      repository: Arc<RwLock<dyn Repository>>,
      event_bus: Arc<RwLock<dyn EventBus>>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            queue: Arc::new(RwLock::new(PriorityQueue::new())),
        }
    }
}

#[async_trait]
impl CommandBus for MemoryCommandBus {
    async fn publish(&mut self, command: Commands) {
        let priority = match command {
            Commands::Domain(ref command) => command.priority.clone(),
        };

        self.queue.write().await.push(command, priority as u8);
    }

    async fn run(&mut self) {
        let queue = self.queue.clone();
        let repo = self.repository.clone();
        let event_bus = self.event_bus.clone();
        // Spawn thread to consume commands
        tokio::spawn(async move {
            loop {
                match queue.write().await.pop() {
                    Some((command, _priority)) => {
                        let repo = repo.clone();
                        let event_bus = event_bus.clone();
                        tokio::spawn(async move {
                            match command {
                                Commands::Domain(mut command) => {
                                    let _ = command.run(repo, event_bus).await;
                                },
                            };
                        });
                    },
                    None => (),
                };
            }
        });
    }
}
