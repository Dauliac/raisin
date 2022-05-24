use crate::core::domain::Entity;
use crate::domain::cfg::cfg::Cfg;
use crate::domain::sources::file::File;
use std::boxed::Box;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Program {
    uuid: Uuid,
    cfgs: HashMap<String, Cfg>,
    sources: HashMap<String, File>,
}

impl Entity for Program {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            cfgs: HashMap::new(),
            sources: HashMap::new(),
        }
    }
}
