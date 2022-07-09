use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::{IterMut, Iter}, HashMap};
use std::hash::Hash;

use super::file::File;
use crate::core::domain::{new_uuid, Aggregate, Entity, Uuid, Event};
use crate::domain::program::Language;


#[derive(Error, Debug)]
pub enum SourcesError {
    #[error("given dir `${path:?}` not exists")]
    FileNotIndexed { path: String },
}

#[derive(PartialEq, Eq, Hash)]
pub enum SourcesEvent {
    SourcesDeclared {
        sources_uuid: Uuid,
        language: Language,
    },
    FileIndexed {
        file_uuid: Uuid,
        path: String,
    },
    FileNotIndexed {
    },
    FileContentLoaded {
        file_uuid: Uuid,
    },
}

impl Event for SourcesEvent {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    uuid: Uuid,
    files: HashMap<Uuid, File>,
    language: Language,
}

impl Entity for Sources {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Aggregate for Sources {
    type Error = SourcesError;
    type Event = SourcesEvent;
    type Result = Result<Vec<Self::Event>, Self::Error>;
}

impl Sources {

    pub fn new(uuid: Option<Uuid>, language: Language) -> Self {
        Self {
            uuid: match uuid {
                Some(uuid) => uuid,
                None => new_uuid(),
            },
            files: HashMap::new(),
            language,
        }
    }

    pub fn register(language: Language) -> Self {
        Self::new(None, language)
    }

    pub async fn index_new_file(&mut self, file_path: String) -> <Self as Aggregate>::Event {
        let file = File::new(None, file_path, self.language.clone());
        let file_uuid = file.get_uuid();
        let path = file.get_path();
        self.files.insert(file_uuid, file);

        SourcesEvent::FileIndexed {
            file_uuid,
            path,
        }
    }

    pub fn get_files(&self) -> Iter<Uuid, File> {
        self.files.iter()
    }

    pub fn edit_files(&mut self) -> IterMut<Uuid, File> {
        self.files.iter_mut()
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
}
