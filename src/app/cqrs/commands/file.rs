use std::rc::Rc;
use thiserror::Error;
use uuid::Uuid;

use crate::app::cqrs::Query;
use crate::app::dtos::sources::Sources as SourcesDTO;
use crate::core::domain::Entity;
use crate::infra::services::sources::source_reader::{Error as InfraSourcesError, SourceReader};

// Right side Query
pub struct DiscoverSources {
    uuid: Uuid,
    path: String,
    pub service: Option<Rc<SourceReader>>,
    result: Option<Result<SourcesDTO, DiscoverSourcesError>>,
}

impl Entity for DiscoverSources {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl DiscoverSources {
    pub fn new(path: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            path,
            service: None,
            result: None,
        }
    }
    pub fn get_path(&self) -> String {
        self.path
    }
}

#[derive(Error, Debug)]
pub enum DiscoverSourcesError {
    #[error("Infrastructure error")]
    Infra(InfraSourcesError),
}

impl Query for DiscoverSources {
    fn run(&mut self) -> Result<SourcesDTO, DiscoverSourcesError> {
        let res = match self.service.unwrap().run() {
            Ok(result) => Ok(result),
            Err(err) => DiscoverSourcesError::Infra(err),
        };
        self.result = Some(res);

        self.result.unwrap()
    }
}

pub enum RightSourcesCommands {
    DiscoverFiles(DiscoverSources),
}

// pub enum SourcesQueries {
//     ReadSources(ReadSources),
// }

pub struct SourcesQueryFactory {}
impl SourcesQueryFactory {
    pub fn discover_files(path: String) -> RightSourcesCommands {
        let query = DiscoverSources::new(path);
        RightSourcesCommands::DiscoverFiles(query)
    }
}

// pub struct SourcesQueriesFactory {}
// impl SourcesQueriesFactory {
//     pub fn discover_files(path: String) -> SourcesQueries {
//         let query = ReadSources::new(path);

//         SourcesQueries::ReadSources(query)
//     }
// }
