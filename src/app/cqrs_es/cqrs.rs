use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    core::domain::Aggregate,
    domain::{
        program::{Program, ProgramCommand},
        repository::Repository,
    },
};

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

    pub async fn run(&mut self, repository: Arc<RwLock<dyn Repository>>) -> <Program as Aggregate<Program>>::Result {
        let mut writable_locked_repo = repository.write().unwrap();
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
                    let mut writable_program = match program.write() {
                        Ok(program) => program,
                        Err(error) => {
                            panic!("Failed to write lock program");
                        }
                    };
                    result = writable_program.handle(self.command.clone()).await;

                    match &result {
                        Ok(events) => {
                            let events = events.clone();
                            for event in events {
                                writable_program.apply(event);
                            }
                        },
                        Err(_) => {}
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
