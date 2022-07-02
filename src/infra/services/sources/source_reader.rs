use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

use crate::core::domain::Aggregate;
use crate::domain::program::Language;
use crate::domain::sources::aggregate::Sources;

pub struct Config {
    pub path: PathBuf,
    pub language: Language,
}

pub struct SourceReader {
    config: Config,
}

impl SourceReader {
    pub fn new(conf: Config) -> SourceReader {
        SourceReader { config: conf }
    }

    pub async fn run(&self) -> Result<(Sources, Vec<<Sources as Aggregate>::Event>), Error> {
        let mut failed_files_path = Vec::new();
        let mut sources = Sources::register(self.config.language.clone());
        let mut register_events: Vec<<Sources as Aggregate>::Event> = Vec::new();

        let path = self.config.path.clone();
        if !path.exists() {
            return Err(Error::NotExists(path.to_str().unwrap().to_string()));
        }

        for file_path in WalkDir::new(path.to_str().unwrap())
            .into_iter()
            .filter_map(|file| file.ok())
        {
            let metadata = file_path.metadata();
            let path = path.to_str().unwrap().to_string();

            match metadata {
                Err(_) => {
                    failed_files_path.push(path);
                    continue;
                }
                Ok(metadata) => {
                    if metadata.is_file() {
                        let file = sources.index_new_file(path).await;
                        register_events.append(&mut vec![file]);
                    }
                }
            }
        }

        if path.is_dir() && sources.get_files().len() == 0 {
            return Err(Error::EmptyDir(path.to_str().unwrap().to_string()));
        }

        if !failed_files_path.is_empty() {
            return Err(Error::FailedToIndexFiles {
                sources,
                failed_files_path,
            });
        }

        return Ok((sources, register_events));
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to register sources: ${0}")]
    DomainError(<Sources as Aggregate>::Error),
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
