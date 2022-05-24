use crate::core::domain::Entity;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Scope {
    uuid: Uuid,
    parent: Option<Box<Scope>>,
    childs: HashMap<String, Box<Scope>>,
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
            childs: HashMap::new(),
        }
    }

    pub fn get_parent(&self) -> &Option<Box<Self>> {
        &self.parent
    }

    pub fn set_parent(&mut self, parent: Box<Self>) {
        self.parent = Some(parent);
    }

    pub fn set_child(&mut self, child: Box<Self>) -> Option<Box<Self>> {
        self.childs.insert(child.uuid.to_string(), child)
    }
}
