// use super::code::Code;
use crate::core::domain::{new_uuid, Entity, Uuid};
use crate::domain::program::Language;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter as HashMapIter;
use std::collections::hash_set::Iter as HashSetIter;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct File {
    uuid: Uuid,
    pub path: String,
    lines: HashMap<u64, String>,
    language: Language,
    includes: HashSet<String>,
}

impl Entity for File {
    fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl File {
    pub fn new(uuid: Option<Uuid>, path: String, language: Language) -> File {
        Self {
            uuid: match uuid {
                Some(uuid) => uuid,
                None => new_uuid(),
            },
            path,
            language,
            lines: HashMap::new(),
            includes: HashSet::new(),
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn insert_line(&mut self, line_number: u64, line: String) {
        self.lines.insert(line_number, line);
    }

    pub fn get_lines(&self) -> HashMapIter<u64, String> {
        self.lines.iter()
    }

    pub fn include(&mut self, uuid: String) {
        self.includes.insert(uuid);
    }

    pub fn get_includes(&self) -> HashSetIter<String> {
        self.includes.iter()
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
    // pub fn insert_content(&mut self, code: Code) {
    //     self.lines.insert(code.coordinate.start_line, code);
    // }
}
