use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

use crate::app::cqrs::Command;
use crate::app::dtos::sources::File as FileDTO;
use crate::app::dtos::Cfg as CfgDTO;
use crate::app::dtos::Uuid as UuidDTO;
use crate::core::domain::Entity;
use crate::infra::services::parsers::Error;
use crate::infra::services::parsers::Parser;

pub struct CfgWasGenerated {
    uuid: Uuid,
    pub file: FileDTO,
    pub service: Option<Rc<dyn Parser>>,
    // TODO: dauliac add Result into commands
    pub result: Option<Result<HashMap<UuidDTO, CfgDTO>, Error>>,
}

impl Entity for CfgWasGenerated {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl CfgWasGenerated {
    pub fn new(file: FileDTO) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            file,
            service: None,
            result: None,
        }
    }
}

pub enum CfgCommand {
    CfgWasGenerated(CfgWasGenerated),
}

impl Command for CfgWasGenerated {
    fn run(&mut self) {
        // self.result = Some((self.callback)());
        self.result = match &self.service {
            Some(service) => Some(service.run(&self.file)),
            None => None,
        };
    }
}
