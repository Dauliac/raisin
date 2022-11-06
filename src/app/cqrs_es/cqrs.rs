use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::SystemTime,
};
use tokio::sync::RwLock;

use crate::{
    core::domain::Aggregate,
    domain::{
        program::{Program, ProgramCommand},
        repository::Repository,
    }, infra::services::bus::event_bus,
};

use super::event::{EventBus, Events};

#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandPriority {
    Low = 30,
    Normal = 60,
    High = 90,
}

#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Hash, Clone)]
pub struct Command<C> {
    pub command: C,
    pub timestamp: Option<SystemTime>,
    pub priority: CommandPriority,
}
type DomainCommand = Command<<Program as Aggregate<Program>>::Command>;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Commands {
    Domain(DomainCommand),
}

impl Commands {
    pub fn new_domain(command: <Program as Aggregate<Program>>::Command) -> Self {
        let command = DomainCommand::new(command);
        Commands::Domain(command)
    }
}


impl DomainCommand {
    pub fn new(
        command: <Program as Aggregate<Program>>::Command,
    ) -> Self {
        let priority = match command {
            ProgramCommand::DiscoverProgram(_) => {
                CommandPriority::High
            },
            _ => {
                CommandPriority::Normal
            },
        };

        Self {
            command,
            timestamp: None,
            priority
        }
    }

    pub async fn run(
      &mut self,
      repository: Arc<RwLock<dyn Repository>>,
      event_bus: Arc<RwLock<dyn EventBus>>
    ) -> <Program as Aggregate<Program>>::Result {

        let mut writable_locked_repo = repository.write().await;
        self.timestamp = Some(SystemTime::now());
        let result: <Program as Aggregate<Program>>::Result;
        let command = self.command.clone();
        match command {
            ProgramCommand::DiscoverProgram(command) => {
                let discover = Program::discover(command);
                let program = Arc::new(RwLock::new(discover.0));
                writable_locked_repo.write(program);

                result = discover.1;

                return result;
            },
            _ => {
                let program = match writable_locked_repo.read() {
                    None => {
                        panic!("No program loaded in the repository");
                    },
                    Some(program) => program,
                };
                {
                    // Lock program
                    let mut writable_program = program.write().await;
                    result = writable_program.handle(self.command.clone()).await;
                    let mut writable_event_bus = event_bus.write().await;

                    match &result {
                        Ok(events) => {
                            let events = events.clone();
                            for event in events {
                                writable_program.apply(event.clone());
                                let app_event = Events::new_domain(event);
                                writable_event_bus.publish(app_event);
                            }
                        },
                        Err(error) => {
                            let app_event = Events::new_domain_error(error.clone());
                            writable_event_bus.publish(app_event);
                        }
                    }
                } // Free program write lock

                writable_locked_repo.write(program);

                return result;
            }
        }
    }
}

#[async_trait]
pub trait CommandBus {
    async fn publish(&mut self, command: Commands);
    async fn run(&mut self);
}
