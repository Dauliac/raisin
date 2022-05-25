// use super::code::Code;
use crate::core::domain::Entity;

use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct File {
    uuid: Uuid,
    path: PathBuf,
    lines: HashMap<u64, String>,
    includes: Vec<Uuid>,
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
    pub fn new(path: PathBuf) -> File {
        Self {
            uuid: Uuid::new_v4(),
            path,
            lines: HashMap::new(),
            includes: Vec::new(),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.to_owned()
    }
    // pub fn insert_content(&mut self, code: Code) {
    //     self.lines.insert(code.coordinate.start_line, code);
    // }
}
