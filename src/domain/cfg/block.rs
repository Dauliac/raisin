use crate::core::domain::Entity;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Block {
    uuid: Uuid,
    precedents: HashMap<String, Box<Block>>,
    successors: HashMap<String, Box<Block>>,
}

impl Entity for Block {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            precedents: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    pub fn add_precedent(&mut self, precedent: Box<Self>) -> Option<Box<Self>> {
        self.precedents
            .insert(precedent.uuid.to_string(), precedent)
    }

    pub fn get_precedent(&self, precedent: &Uuid) -> Option<&Box<Self>> {
        self.precedents.get(&precedent.to_string())
    }

    pub fn add_successor(&mut self, successor: Box<Self>) -> Option<Box<Self>> {
        self.successors
            .insert(successor.uuid.to_string(), successor)
    }

    pub fn get_successor(&self, successor: &Uuid) -> Option<&Box<Self>> {
        self.successors.get(&successor.to_string())
    }
}
