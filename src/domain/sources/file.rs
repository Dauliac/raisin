// use super::code::Code;
use crate::{core::domain::Entity, domain::program::Language};
use std::collections::hash_map::Iter as HashMapIter;
use std::collections::hash_set::Iter as HashSetIter;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct File {
    uuid: Uuid,
    pub path: PathBuf,
    lines: HashMap<u64, String>,
    language: Language,
    includes: HashSet<Uuid>,
}

impl Entity for File {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl File {
    pub fn new(uuid: Option<Uuid>, path: PathBuf, language: Language) -> File {
        Self {
            uuid: match uuid {
                Some(uuid) => uuid,
                None => Uuid::new_v4(),
            },
            path,
            language,
            lines: HashMap::new(),
            includes: HashSet::new(),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.to_owned()
    }

    pub fn insert_line(&mut self, line_number: u64, line: String) -> Option<String> {
        self.lines.insert(line_number, line)
    }

    pub fn get_lines(&self) -> HashMapIter<u64, String> {
        self.lines.iter()
    }

    pub fn include(&mut self, uuid: Uuid) -> bool {
        self.includes.insert(uuid)
    }

    pub fn get_includes(&self) -> HashSetIter<Uuid> {
        self.includes.iter()
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
    // pub fn insert_content(&mut self, code: Code) {
    //     self.lines.insert(code.coordinate.start_line, code);
    // }
}
