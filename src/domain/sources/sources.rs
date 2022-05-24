use std::{
    collections::{hash_map::Iter, HashMap},
    path::PathBuf,
};
use thiserror::Error;
use uuid::Uuid;

use super::file::File;
use crate::core::domain::Entity;

#[derive(Debug, Clone)]
pub struct Sources {
    uuid: Uuid,
    pub files: HashMap<Uuid, File>,
}

impl Entity for Sources {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Sources {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            files: HashMap::new(),
        }
    }

    pub fn index_sources(&mut self, file_path: PathBuf) {
        let file = File::new(file_path);
        self.files.insert(file.get_uuid(), file);
    }

    pub fn iter(&self) -> Iter<Uuid, File> {
        self.files.iter()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("given sources `${0}` not exists")]
    NotExists(PathBuf),
    #[error("given dir `${0}` is empty")]
    EmptyDir(PathBuf),
    #[error("Unable to read current dir `${0}`")]
    PathNotReadable(PathBuf),
    #[error("Unable to index some files `{failed_files:?}`")]
    FailedToIndexFiles {
        files: Vec<String>,
        failed_files: Vec<String>,
    },
}
