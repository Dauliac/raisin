use crate::{core::domain::Entity, core::domain::Uuid};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Scope {
    uuid: Uuid,
    parent: Option<String>,
    childs: HashSet<String>,
}

impl Entity for Scope {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            parent: None,
            childs: HashSet::new(),
        }
    }

    pub fn get_parent(&self) -> &Option<String> {
        &self.parent
    }

    pub fn set_parent(&mut self, parent_uuid: String) {
        self.parent = Some(parent_uuid);
    }

    pub fn set_child(&mut self, child_uuid: String) -> bool {
        self.childs.insert(child_uuid)
    }
}
