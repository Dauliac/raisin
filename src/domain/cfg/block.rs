use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::core::domain::{Entity, Uuid};

#[derive(Serialize, Deserialize)]
pub struct Block {
    uuid: Uuid,
    precedents: HashSet<String>,
    successors: HashSet<String>,
}

impl Entity for Block {
    fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            precedents: HashSet::new(),
            successors: HashSet::new(),
        }
    }

    pub fn add_precedent(&mut self, precedent_uuid: String) -> bool {
        self.precedents.insert(precedent_uuid)
    }

    pub fn get_precedent(&self, precedent_uuid: &String) -> Option<&String> {
        self.precedents.get(precedent_uuid)
    }

    pub fn add_successor(&mut self, successor_uuid: String) -> bool {
        self.successors.insert(successor_uuid)
    }

    pub fn get_successor(&self, successor_uuid: &String) -> Option<&String> {
        self.successors.get(successor_uuid)
    }
}
