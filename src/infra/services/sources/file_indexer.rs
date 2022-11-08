use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use walkdir::WalkDir;

use crate::app::cqrs_es::cqrs::{CommandBus, Commands};
use crate::core::domain::Aggregate;
use crate::domain::program::Program;
use crate::domain::sources::aggregate::Sources;

pub struct Config {
    pub program: Arc<RwLock<Program>>,
    pub command_bus: Arc<RwLock<dyn CommandBus>>,
}

pub struct FileIndexer {
    config: Config,
}

impl FileIndexer {
    pub fn new(conf: Config) -> FileIndexer {
        FileIndexer { config: conf }
    }

    pub async fn run(&self) {
        let program = self.config.program.write().await;
        let sources_path = program.get_path();
        // if !path.exists() {
        //     return Err(Error::NotExists(path.to_str().unwrap().to_string()));
        // }

        for file_path in WalkDir::new(sources_path.to_str().unwrap())
            .into_iter()
            .filter_map(|file| file.ok())
        {
            let metadata = file_path.metadata();
            let file_path = file_path.path().to_owned();


            match metadata {
                Err(_) => {
                    todo!("Implement Event for infra and file_indexer");
                    continue;
                }
                Ok(metadata) => {
                    if metadata.is_file() {
                        println!("go {:?}", file_path.to_str());
                        let command = Program::index_new_file(file_path);
                        let command = Commands::new_domain(command);
                        let mut command_bus = self.config.command_bus.write().await;

                        command_bus.publish(command).await;
                    }
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to register sources: ${0}")]
    DomainError(<Program as Aggregate<Program>>::Error),
    #[error("given dir `${0}` not exists")]
    NotExists(String),
    #[error("given dir `${0}` is empty")]
    EmptyDir(String),
    #[error("Unable to read current dir `${0}`")]
    PathNotReadable(String),
    #[error("Unable to index some files `{failed_files_path:?}`")]
    FailedToIndexFiles {
        sources: Sources,
        failed_files_path: Vec<String>,
    },
}
