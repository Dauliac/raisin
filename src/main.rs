pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use std::rc::Rc;

use app::cqrs::queries::sources::{SourcesOk, SourcesQueryFactory};
use domain::program::AvailableLanguages;
use domain::repository::Repository;
use infra::repositories::sources::Config as SourcesRepositoryConfig;
use infra::services::parsers::AvailableParsers;

use infra::{
    repositories::sources::SourcesRepository,
    services::sources::source_reader::{Config, SourceReader},
};

fn main() {
    let _program = domain::program::Program::new();
    let language = AvailableLanguages::rust();
    let config = Config {
        language: language.clone(),
        path: "./src".to_string(),
    };
    let service = Rc::new(SourceReader::new(config));
    let path = "./src".to_string();
    let config = SourcesRepositoryConfig {
        service,
        path: path.clone(),
        language,
    };
    let mut repo = SourcesRepository::new(config);
    let query = SourcesQueryFactory::discover_files(path);
    // Chaneg it to read
    let result = repo.read(query);

    let parser = AvailableParsers::tree_sitter();
    let result = match result {
        Ok(ok) => ok,
        Err(_) => SourcesOk::Nothing,
    };
    println!("{:?}", result);

    let result = match result {
        SourcesOk::DiscoverSources(sources) => {
            for (uuid, file) in sources.files.iter() {
                parser.run(file);
            }
            Some(())
        }
        SourcesOk::Nothing => None,
    };
}
