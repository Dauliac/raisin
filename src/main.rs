pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use std::path::Path;
use std::rc::Rc;

use app::cqrs::commands::file::{FilesWereDiscovered, SourcesCommand};
use app::cqrs::RamCommandHandler;
use domain::repository::Repository;
use infra::repositories::sources::Config as SourcesRepositoryConfig;
use infra::{
    repositories::sources::SourcesRepository,
    services::sources::source_reader::{Config, SourceReader},
};

fn main() {
    let _program = domain::program::Program::new();
    let config = Config {
        path: "./src".to_string(),
    };
    let service = Rc::new(SourceReader::new(config));
    let command_handler = Box::new(RamCommandHandler::new());
    let path = "./src".to_string();
    let config = SourcesRepositoryConfig {
        service,
        path: path.clone(),
        command_handler,
    };
    let mut repo = SourcesRepository::new(config);
    let command = FilesWereDiscovered::new(path);
    let c = SourcesCommand::DiscoverFiles(command);

    repo.write(c);

    // println!("TADARM {:?}", command)
    // let files = match service.run() {
    //     Ok(files) => println!("files:\n {:?}", files),
    //     Err(_) => return,
    // };
}
