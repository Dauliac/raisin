use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::app::dtos::sources::File as FileDTO;
use crate::app::dtos::sources::Path as PathDTO;
use crate::app::dtos::sources::Sources as SourcesDTO;
use crate::app::dtos::sources::Uuid as UuidDTO;

pub struct Config {
    pub path: String,
}

pub struct SourceReader {
    config: Config,
}

impl SourceReader {
    pub fn new(conf: Config) -> SourceReader {
        SourceReader { config: conf }
    }

    pub fn run(&self) -> Result<SourcesDTO, Error> {
        let mut files = HashMap::new();
        let mut failed_files = HashMap::new();

        let path = Path::new(self.config.path.as_str());
        if !path.exists() {
            return Err(Error::NotExists(path.to_str().unwrap().to_string()));
        }
        for file in WalkDir::new(self.config.path.as_str())
            .into_iter()
            .filter_map(|file| file.ok())
        {
            let metadata = file.metadata();
            let path = path.to_str().unwrap().to_string();
            let file = FileDTO {
                uuid: Uuid::new_v4().to_string(),
                path,
                lines: HashMap::new(),
                includes: Vec::new(),
            };

            if metadata.is_err() {
                failed_files.insert(file.uuid.clone(), file);
                continue;
            }

            let metadata = metadata.unwrap();
            if metadata.is_file() {
                files.insert(file.uuid.clone(), file);
            }
        }

        if path.is_dir() && files.is_empty() {
            return Err(Error::EmptyDir(path.to_str().unwrap().to_string()));
        }

        if !failed_files.is_empty() {
            return Err(Error::FailedToIndexFiles {
                files,
                failed_files,
            });
        }

        let indexed_files = HashMap::new();
        let sources = SourcesDTO {
            uuid: Uuid::new_v4().to_string(),
            files: indexed_files,
        };
        return Ok(sources.to_owned());
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("given dir `${0}` not exists")]
    NotExists(PathDTO),
    #[error("given dir `${0}` is empty")]
    EmptyDir(PathDTO),
    #[error("Unable to read current dir `${0}`")]
    PathNotReadable(PathDTO),
    #[error("Unable to index some files `{failed_files:?}`")]
    FailedToIndexFiles {
        files: HashMap<UuidDTO, FileDTO>,
        failed_files: HashMap<UuidDTO, FileDTO>,
    },
}
