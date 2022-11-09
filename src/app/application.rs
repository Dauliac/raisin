use clap::{Command, CommandFactory};
use clap_complete::{generate, Generator};
use tokio::sync::RwLock;
use std::{io, sync::Arc};
use std::path::PathBuf;

use super::cqrs_es::cqrs::{CommandBus, Commands};
use super::cqrs_es::event::EventBus;
use super::handlers::subscribe_logger;
use super::services::Service;
use super::services::cli::{ArgParserService, Cli, Commands as CliCommands , Language as CliLanguage};
use super::services::logger::Logger;
use crate::domain::languages::Languages;
use crate::domain::program::{ProgramCommand, DiscoverProgram};
use crate::domain::repository::Repository;
use crate::infra::repositories::RepositoryInMemory;
use crate::infra::services::bus::cqrs::MemoryCommandBus;
use crate::infra::services::bus::event_bus::MemoryEventBus;
use crate::infra::services::adapters::sources::file_indexer::{FileIndexer, Config as FileIndexerConfig};
use crate::{domain::languages::AvailableLanguages, infra::services::logger::SimpleLogger};

pub struct Application {
    cli: Option<Cli>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.name().to_string(), &mut io::stdout());
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
        let logger = Arc::new(RwLock::new(SimpleLogger::new()));
        match &cli.command {
            CliCommands::Complete { shell } => {
                eprintln!("Generating completion file for {:?}...", shell);
                let mut cmd = Cli::command();
                print_completions(shell.clone(), &mut cmd);
            }
            CliCommands::Parse { path, language, daemon } => {
                let language = match language {
                    CliLanguage::Rust => AvailableLanguages::rust(),
                    _ => AvailableLanguages::rust(),
                };

                let path: PathBuf = path.to_owned();
                let repository: Arc<RwLock<dyn Repository>> = Arc::new(RwLock::new(RepositoryInMemory::new()));
                if daemon.clone() {
                    self.start_daemon(logger, repository, language, path).await;
                }
            }
        };
    }

    async fn start_daemon(
      &self,
      logger: Arc<RwLock<dyn Logger + Send + Sync>>,
      repository: Arc<RwLock<dyn Repository>>,
      language: Languages,
      path: PathBuf,
    ) {
        let mut event_bus = MemoryEventBus::new();

        subscribe_logger(logger, &mut event_bus).await;
        let event_bus: Arc<RwLock<dyn EventBus + Sync + Send>> = Arc::new(RwLock::new(event_bus));
        event_bus
          .write()
          .await
          .run()
          .await;

        let command_bus: Arc<RwLock<dyn CommandBus>> =
          Arc::new(RwLock::new(MemoryCommandBus::new(repository.clone(), event_bus.clone())));

        let discover_command = DiscoverProgram {
            language,
            path,
        };
        let command = ProgramCommand::DiscoverProgram(discover_command.clone());
        let command = Commands::new_domain(command);
        {
            let mut command_bus = command_bus.write().await;
            command_bus.publish(command).await;
        }

        // manually consume events while program is not saved in repository
        while repository.read().await.read().is_none() {
            {
                let mut event_bus = event_bus.write().await;
                let _ = event_bus.run().await;
            }
            {
                let mut command_bus = command_bus.write().await;
                let _ = command_bus.run().await;
            }
        }

        self.index_sources(repository.clone(), command_bus.clone()).await;

        loop {
            {
                let mut event_bus = event_bus.write().await;
                let _ = event_bus.run().await;
            }
            {
                let mut command_bus = command_bus.write().await;
                let _ = command_bus.run().await;
            }
        };
    }
    async fn index_sources(&self,
      repository: Arc<RwLock<dyn Repository>>,
      command_bus: Arc<RwLock<dyn CommandBus>>,
    ) {
        let program = repository.read()
          .await
          .read()
          .expect("Program was not indexed it app::index_sources call");

        let config = FileIndexerConfig {
            program,
            command_bus,
        };
        let file_indexer_service = FileIndexer::new(config);
        file_indexer_service
          .run()
          .await;
    }
}
