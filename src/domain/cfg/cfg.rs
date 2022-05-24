use super::block::Block;
use super::scope::Scope;
use crate::core::domain::Entity;
use std::boxed::Box;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Cfg {
    uuid: Uuid,
    blocks: HashMap<String, Box<Block>>,
    scopes: HashMap<String, Box<Scope>>,
}

impl Entity for Cfg {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Cfg {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            blocks: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) -> Option<Box<Block>> {
        let id = block.get_uuid().to_string();
        let block = Box::new(block);
        self.blocks.insert(id, block)
    }

    // pub fn get_successor(&self, successor: &Uuid) -> Option<&Box<Self>> {
    //     self.successors.get(&successor.to_string())
    // }
}
