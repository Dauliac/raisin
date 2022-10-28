use clap::{Command, CommandFactory};
use clap_complete::{generate, Generator};
use std::sync::RwLock;
use std::{io, sync::Arc};
use std::path::PathBuf;

use super::cqrs_es::cqrs::{CommandBus, Commands};
use super::cqrs_es::event::{EventBus, Events};
use super::services::Service;
use super::services::cli::{ArgParserService, Cli, Commands as CliCommands , Language as CliLanguage};
use crate::domain::program::{Program, ProgramCommand, DiscoverProgram};
use crate::domain::repository::Repository;
use crate::infra::repositories::RepositoryInMemory;
use crate::infra::services::bus::cqrs::MemoryCommandBus;
use crate::infra::services::bus::event_bus::MemoryEventBus;
use crate::{domain::languages::AvailableLanguages, infra::services::logger::SimpleLogger};

pub struct Application {
    cli: Option<Cli>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

impl Application {
    pub fn new() -> Self {
        Self { cli: None }
    }

    pub async fn read_config(&mut self) {
        let mut arg_svc = ArgParserService::new();
        self.cli = Some(arg_svc.run().await);
    }

    pub async fn start(&self) {
        let cli = self.cli.as_ref().unwrap();
        let logger = SimpleLogger::new();
        match &cli.command {
            CliCommands::Complete { shell } => {
                eprintln!("Generating completion file for {:?}...", shell);
                let mut cmd = Cli::command();
                print_completions(shell.clone(), &mut cmd);
            }
            CliCommands::Parse { path, language } => {
                let language = match language {
                    CliLanguage::Rust => AvailableLanguages::rust(),
                    _ => AvailableLanguages::rust(),
                };

                let path: PathBuf = path.to_owned();
                let repository: Arc<RwLock<dyn Repository>> = Arc::new(RwLock::new(RepositoryInMemory::new()));

                let command_bus: Arc<RwLock<dyn CommandBus>> = Arc::new(RwLock::new(MemoryCommandBus::new(repository)));
                let event_bus: Arc<RwLock<dyn EventBus>> = Arc::new(RwLock::new(MemoryEventBus::new()));

                // Use case one: load program
                let discover_command = DiscoverProgram {
                    language,
                    path,
                };
                let command = ProgramCommand::DiscoverProgram(discover_command.clone());
                let command = Commands::new_domain(command);
                match command_bus.write() {
                    Ok(mut command_bus) => {
                        command_bus.publish(command);
                    },
                    Err(error) => {
                        todo!("Log error");
                        // panic!("Failed to write lock command_bus");
                    }
                };
                let (program, events ) = Program::discover(discover_command);

                match event_bus.write() {
                    Ok(mut event_bus) => {
                        match events {
                            Ok(events) => {
                                for event in events {
                                    let event = Events::new_domain(event);
                                    event_bus.publish(event);
                                }
                            },
                            Err(invalid_use_case) => {
                                let event = Events::new_domain_error(invalid_use_case);
                                event_bus.publish(event);
                            }
                        }
                    },
                    Err(error) => {
                        todo!("Log error");
                        // panic!("Failed to write lock event_bus");
                    }
                };

                // Env of use case

                // let mut load_project_service = LoadProjectService::new(config);
                // let sources_repo = load_project_service.run().await;
                // let sources = sources_repo.read().unwrap();
                // let parse_projet_config = ParseProjectConfig {
                //     sources,
                //     event_bus: event_bus.clone(),
                // };
                // let mut parse_project_service = ParseProjectService::new(parse_projet_config);
                // let _cfg_repo = parse_project_service.run().await;
            }
        };
    }
}
