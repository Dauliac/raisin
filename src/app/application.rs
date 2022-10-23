use clap::{Command, CommandFactory};
use clap_complete::{generate, Generator};
use std::io;
use std::path::PathBuf;

use super::services::{
    cli_service::{ArgParserService, Cli, Commands, Language as CliLanguage},
    Service,
};
// use crate::app::services::parse_project_service::{ParseProjectService, Config as ParseProjectConfig};
// use crate::app::services::load_project_service::{LoadProjectService, Config as LoadProjectConfig };
// use crate::infra::event_bus::MemoryEventBus;
use crate::domain::languages::AvailableLanguages;

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
        match &cli.command {
            Commands::Complete { shell } => {
                eprintln!("Generating completion file for {:?}...", shell);
                let mut cmd = Cli::command();
                print_completions(shell.clone(), &mut cmd);
            }
            Commands::Parse { path, language } => {
                let language = match language {
                    CliLanguage::Rust => AvailableLanguages::rust(),
                    _ => AvailableLanguages::rust(),
                };

                let path: PathBuf = path.to_owned();
                // let event_bus = Arc::new(MemoryEventBus::new());

                // let config = LoadProjectConfig {
                //   path,
                //   language,
                //   event_bus: event_bus.clone(),
                // };
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
