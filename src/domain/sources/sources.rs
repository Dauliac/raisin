use std::{
    collections::{hash_map::Iter, HashMap},
    path::PathBuf,
};
use uuid::Uuid;

use super::file::File;
use crate::{core::domain::Entity, domain::program::Language};

#[derive(Debug, Clone)]
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

impl Sources {
    pub fn new(uuid: Option<Uuid>, language: Language) -> Self {
        Self {
            uuid: match uuid {
                Some(uuid) => uuid,
                None => Uuid::new_v4(),
            },
            files: HashMap::new(),
            language,
        }
    }

    // TODO check if it's used
    pub fn index_sources(&mut self, file_path: PathBuf) {
        let file = File::new(None, file_path, self.language.clone());
        self.files.insert(file.get_uuid(), file);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.insert(file.get_uuid(), file);
    }

    pub fn get_files(&self) -> Iter<Uuid, File> {
        self.files.iter()
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
}
