use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::core::domain::{new_uuid, Entity, Uuid};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct BlockUuid(Uuid);
impl BlockUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    uuid: BlockUuid,
    precedents: HashSet<BlockUuid>,
    successors: HashSet<BlockUuid>,
}

impl Entity<Self> for Block {
    type Uuid = BlockUuid;

    fn uuid(&self) -> BlockUuid {
        self.uuid.clone()
    }

    fn equals(&self, entity: Box<Self>) -> bool {
        self.uuid == entity.uuid()
    }
}

impl Block {
    pub fn new(uuid: BlockUuid) -> Self {
        Self {
            uuid,
            precedents: HashSet::new(),
            successors: HashSet::new(),
        }
    }

    pub fn add_precedent(&mut self, precedent_uuid: BlockUuid) -> bool {
        self.precedents.insert(precedent_uuid)
    }

    pub fn precedent(&self, precedent_uuid: &BlockUuid) -> Option<&BlockUuid> {
        self.precedents.get(precedent_uuid)
    }

    pub fn add_successor(&mut self, successor_uuid: BlockUuid) -> bool {
        self.successors.insert(successor_uuid)
    }

    pub fn successor(&self, successor_uuid: &BlockUuid) -> Option<&BlockUuid> {
        self.successors.get(successor_uuid)
    }
}
