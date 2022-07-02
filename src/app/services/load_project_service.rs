use crate::infra::{
    repositories::sources::Config as SourcesRepositoryConfig,
    repositories::sources::SourcesRepository,
    services::sources::source_reader::{Config as SourcesReaderConfig, SourceReader},
};

use std::path::PathBuf;
use std::rc::Rc;

use crate::domain::program::Language;

pub struct Config {
    path: PathBuf,
    language: Language,
}

pub struct LoadProjectService {
    config: Config,
    sources_repository: SourcesRepository,
}

impl LoadProjectService {
    pub fn new(config: Config) -> Self {
        let sources_reader_config = SourcesReaderConfig {
            language: config.language.clone(),
            path: config.path.clone(),
        };
        let service = Rc::new(SourceReader::new(sources_reader_config));
        let source_repo_config = SourcesRepositoryConfig {
            service,
            path: config.path.clone(),
            language: config.language.clone(),
        };

        let mut sources_repository = SourcesRepository::new(source_repo_config);

        Self {
            config,
            sources_repository,
        }
    }
}

// impl Service<()> for LoadProjectService {
//     fn run(&mut self) -> () {
//         let query = SourcesQueryFactory::discover_files(path);
//         let result = this.repo.read(query);
//     }
// }
