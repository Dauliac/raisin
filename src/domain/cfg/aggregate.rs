use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};
use std::fmt::{Debug, Display};
use thiserror::Error;

use super::block::{Block, BlockUuid};
use super::scope::{Scope, ScopeUuid};
use crate::core::domain::Event;
use crate::{
    core::domain::{new_uuid, Aggregate, Entity, Uuid},
    domain::sources::code::Code,
};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct CfgUuid(Uuid);
impl CfgUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

impl Display for CfgUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(EnumString, EnumVariantNames, IntoStaticStr, Default, Error, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum CfgError {
    #[default]
    #[error("nothing")]
    Nothing,
    #[error("cfg_not_parsable")]
    CfgNotParsable,
}
impl Event<CfgError> for CfgError {}


#[derive(Debug, Default, EnumString, IntoStaticStr, EnumVariantNames, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[strum(serialize_all = "kebab_case")]
pub enum CfgEvent {
    #[default]
    Unknown,
    CfgDiscovered {
        cfg_uuid: CfgUuid,
    },
    CfgParsed {
        cfg_uuid: CfgUuid,
        code: Code,
    },
    BlockLoaded {
        cfg_uuid: CfgUuid,
        block_uuid: BlockUuid,
        precedents_uuids: Vec<BlockUuid>,
        successors_uuids: Vec<BlockUuid>,
    },
    ScopeLoaded {
        cfg_uuid: CfgUuid,
        scope_uuid: ScopeUuid,
        parent_uuid: Option<ScopeUuid>,
        childs_uuids: Vec<ScopeUuid>,
    },
}
impl Event<CfgEvent> for CfgEvent {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum CfgCommand {
    DiscoverCfg,
    LoadBlock {
        precedents_uuids: Vec<BlockUuid>,
        successors_uuids: Vec<BlockUuid>,
    },
    LoadScope {
        parent_uuid: Option<ScopeUuid>,
        childs_uuids: Vec<ScopeUuid>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    uuid: CfgUuid,
    code: Option<Code>,
    blocks: HashMap<BlockUuid, Box<Block>>,
    scopes: HashMap<ScopeUuid, Box<Scope>>,
}

impl Entity<Self> for Cfg {
    type Uuid = CfgUuid;

    fn get_uuid(&self) -> Self::Uuid {
        self.uuid.clone()
    }
    fn equals(&self, entity: Box<Cfg>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Cfg {
    fn new(uuid: CfgUuid) -> Self {
        Self {
            uuid,
            code: None,
            blocks: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn discover() -> (Self, <Self as Aggregate<Self>>::Result) {
        let uuid = CfgUuid::new();
        let events = vec![CfgEvent::CfgDiscovered {
            cfg_uuid: uuid.clone(),
        }];
        return (Self::new(uuid), Ok(events));
    }

    fn get_block(&self, uuid: &BlockUuid) -> Option<&Box<Block>> {
        self.blocks.get(uuid)
    }

    fn get_scope(&self, uuid: &ScopeUuid) -> Option<&Box<Scope>> {
        self.scopes.get(uuid)
    }

    fn get_code(&self) -> Option<Code> {
        self.code.clone()
    }
}

#[async_trait]
impl Aggregate<Self> for Cfg {
    type Error = CfgError;
    type Event = CfgEvent;
    type Command = CfgCommand;
    type Result = Result<Vec<Self::Event>, Self::Error>;

    async fn handle(&self, command: Self::Command) -> Self::Result {
        let mut events = Vec::new();
        match command {
            Self::Command::DiscoverCfg => {
                let event = CfgEvent::CfgDiscovered {
                    cfg_uuid: self.uuid.clone(),
                };
                events.push(event);
            }
            Self::Command::LoadBlock {
                precedents_uuids,
                successors_uuids,
            } => {
                let block_uuid = BlockUuid::new();
                let event = CfgEvent::BlockLoaded {
                    cfg_uuid: self.uuid.clone(),
                    block_uuid,
                    precedents_uuids,
                    successors_uuids,
                };
                events.push(event);
            }
            Self::Command::LoadScope {
                parent_uuid,
                childs_uuids,
            } => {
                let scope_uuid = ScopeUuid::new();
                let event = CfgEvent::ScopeLoaded {
                    cfg_uuid: self.uuid.clone(),
                    scope_uuid,
                    parent_uuid,
                    childs_uuids,
                };
                events.push(event);
            }
        };

        Ok(events)
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::Unknown => {},
            Self::Event::CfgDiscovered { cfg_uuid } => {
                self.uuid = cfg_uuid;
                self.code = None;
                self.blocks = HashMap::new();
                self.scopes = HashMap::new();
            }
            Self::Event::CfgParsed { cfg_uuid: _, code } => {
                self.code = Some(code);
            }
            Self::Event::BlockLoaded {
                cfg_uuid: _,
                block_uuid,
                precedents_uuids,
                successors_uuids,
            } => {
                let mut block = Block::new(block_uuid);
                for precedent_uuid in precedents_uuids {
                    block.add_precedent(precedent_uuid);
                }
                for successor_uuid in successors_uuids {
                    block.add_successor(successor_uuid);
                }
            }
            Self::Event::ScopeLoaded {
                cfg_uuid: _,
                scope_uuid,
                parent_uuid,
                childs_uuids,
            } => {
                let mut scope = Scope::new(scope_uuid);
                scope.set_parent(parent_uuid);
                for child_uuid in childs_uuids {
                    scope.set_child(child_uuid);
                }
            }
        }
    }
}
