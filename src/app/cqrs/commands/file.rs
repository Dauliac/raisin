use std::rc::Rc;
use uuid::Uuid;

use crate::app::cqrs::Command;
use crate::app::dtos::sources::Sources as SourcesDTO;
use crate::core::domain::Entity;
use crate::infra::services::sources::source_reader::Error;
use crate::infra::services::sources::source_reader::SourceReader;

pub struct FilesWereDiscovered {
    uuid: Uuid,
    pub path: String,
    pub service: Option<Rc<SourceReader>>,
    // TODO: dauliac add Result into commands
    pub result: Option<Result<SourcesDTO, Error>>,
}

impl Entity for FilesWereDiscovered {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl FilesWereDiscovered {
    pub fn new(path: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            path,
            service: None,
            result: None,
        }
    }
}

pub enum SourcesCommand {
    DiscoverFiles(FilesWereDiscovered),
}

impl Command for FilesWereDiscovered {
    fn run(&mut self) {
        // self.result = Some((self.callback)());
        self.result = match &self.service {
            Some(service) => Some(service.run()),
            None => None,
        };
    }
}
