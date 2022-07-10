use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use thiserror::Error;

use super::block::Block;
use super::scope::Scope;
use crate::{core::domain::{new_uuid, Aggregate, Entity, Uuid}, domain::sources::code::Code};

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("given sources was not loaded")]
    CfgNotParsable,
}

#[derive(PartialEq, Eq, Hash)]
pub enum CfgEvent {
    CfgDiscovered,
}

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    uuid: Uuid,
    code: Code,
    blocks: HashMap<Uuid, Box<Block>>,
    scopes: HashMap<Uuid, Box<Scope>>,
}

impl Entity for Cfg {
    fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Aggregate for Cfg {
    type Error = CfgError;
    type Event = CfgEvent;
    type Result = Result<Vec<Self::Event>, Self::Error>;
}

impl Cfg {

    pub fn new(uuid: Option<Uuid>, code: Code) -> Self {
        Self {
            uuid: match uuid {
                Some(uuid) => uuid,
                None => new_uuid(),
            },
            code,
            blocks: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn discover(code: Code) -> (Self, Vec<<Cfg as Aggregate>::Event>) {
        (Self::new(None, code), vec![CfgEvent::CfgDiscovered])
    }

    pub async fn insert_block(&mut self, block: Block) {
        let uuid = block.get_uuid();
        let block = Box::new(block);
        self.blocks.insert(uuid, block);
    }

    // pub fn get_successor(&self, successor: &Uuid) -> Option<&Box<Self>> {
    //     self.successors.get(&successor.to_string())
    // }
}
