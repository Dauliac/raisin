use crate::{
    core::domain::Entity,
    core::domain::{new_uuid, Uuid},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ScopeUuid(Uuid);
impl ScopeUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scope {
    uuid: ScopeUuid,
    parent: Option<ScopeUuid>,
    childs: HashSet<ScopeUuid>,
}

impl Entity<Self> for Scope {
    type Uuid = ScopeUuid;

    fn get_uuid(&self) -> ScopeUuid {
        self.uuid.clone()
    }

    fn equals(&self, entity: Box<Self>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Scope {
    pub fn new(uuid: ScopeUuid) -> Self {
        Self {
            uuid,
            parent: None,
            childs: HashSet::new(),
        }
    }

    pub fn get_parent(&self) -> &Option<ScopeUuid> {
        &self.parent
    }

    pub fn set_parent(&mut self, parent_uuid: Option<ScopeUuid>) {
        self.parent = parent_uuid;
    }

    pub fn set_child(&mut self, child_uuid: ScopeUuid) -> bool {
        self.childs.insert(child_uuid)
    }
}
