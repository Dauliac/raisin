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
    },
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

        self.timestamp = Some(SystemTime::now());
        let command = self.command.clone();
        let (
            program,
            result
        ) = match command {
            ProgramCommand::DiscoverProgram(command) => {
                let discover = Program::discover(command);
                let program = Arc::new(RwLock::new(discover.0));

                let result = discover.1;

                (program, result)
            },
            _ => {
                println!("command {:?}", &self.command);
                let program = match repository.read().await.read() {
                    None => {
                        panic!("No program loaded in the repository");
                    },
                    Some(program) => program,
                };

                let writable_program = program.write().await;
                let result = writable_program.handle(self.command.clone()).await;
                drop(writable_program);

                (program, result)
            },
        };
        let mut writable_event_bus = event_bus.write().await;

        match &result {
            Ok(events) => {
                let events = events.clone();
                let mut write_program = program.write().await;
                for event in events {
                    write_program.apply(event.clone());

                    let app_event = Events::new_domain(event);
                    writable_event_bus.publish(app_event).await;
                }

            },
            Err(error) => {
                let app_event = Events::new_domain_error(error.clone());
                writable_event_bus.publish(app_event).await;
            }
        };

        let mut writable_locked_repo = repository.write().await;
        writable_locked_repo.write(program);

        return result;
    }
}

#[async_trait]
pub trait CommandBus: Sync + Send {
    async fn publish(&mut self, command: Commands);
    async fn run(&mut self);
}
