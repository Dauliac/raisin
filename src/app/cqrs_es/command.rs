use async_trait::async_trait;

use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::{
    core::domain::Aggregate,
    domain::{
        program::{Program, ProgramCommand, ProgramEvent},
        repository::Repository,
    },
};

pub struct Command<R: Repository> {
    repository: Arc<RwLock<R>>,
    command: <Program as Aggregate<Program>>::Command,
    timestamp: Option<SystemTime>,
}

impl<R: Repository> Command<R> {
    pub fn new(
        command: <Program as Aggregate<Program>>::Command,
        repository: Arc<RwLock<R>>,
    ) -> Self {
        Self {
            repository,
            command,
            timestamp: None,
        }
    }

    pub async fn run(&mut self) -> <Program as Aggregate<Program>>::Result {
        let mut writable_locked_repo = self.repository.write().unwrap();
        self.timestamp = Some(SystemTime::now());
        let result: <Program as Aggregate<Program>>::Result;
        match self.command.clone() {
            ProgramCommand::DeclareProgram(command) => {
                let discover = Program::discover(command);
                let program = Arc::new(RwLock::new(discover.0));
                writable_locked_repo.write(program);

                return discover.1;
            }
            _ => {
                let program = match writable_locked_repo.read() {
                    None => {
                        !todo!("Raise fatal application error");
                    }
                    Some(program) => program,
                };
                {
                    let mut writable_program = program.write().unwrap();
                    result = writable_program.handle(self.command.clone()).await;

                    match result {
                        Ok(events) => {
                            for event in events {
                                writable_program.apply(event);
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                writable_locked_repo.write(program);
            }
        }

        return Ok(Vec::new());
    }
}

// pub enum Commands {
//     Domain(Command),
// }

#[async_trait]
pub trait CommandBus<R: Repository> {
    async fn publish(&self, event: Command<R>);
    async fn run(&mut self);
}
