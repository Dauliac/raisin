use std::rc::Rc;
use thiserror::Error;
use uuid::Uuid;

use crate::app::cqrs::Query;
use crate::app::dtos::sources::SourcesDTO;
use crate::core::domain::Entity;
use crate::infra::services::sources::source_reader::{Error as InfraSourcesError, SourceReader};

// Right side Query
#[derive(Error, Debug)]
pub enum DiscoverSourcesError {
    #[error("Infrastructure error")]
    Infra(InfraSourcesError),
}

#[derive(Error, Debug)]
pub enum SourcesError {
    #[error("Failed to discover sources")]
    DiscoverSources(DiscoverSourcesError),
}

#[derive(Debug)]
pub enum SourcesOk {
    DiscoverSources(SourcesDTO),
    // TODO remove nothing
    Nothing,
}

pub type SourcesResult = Result<SourcesOk, SourcesError>;

pub struct DiscoverSources {
    uuid: Uuid,
    path: String,
    pub service: Option<Rc<SourceReader>>,
    result: Option<Result<SourcesOk, DiscoverSourcesError>>,
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

    // pub fn get_path(&self) -> String {
    //     self.path.clone()
    // }
}

impl Query<SourcesResult> for DiscoverSources {
    fn run(&mut self) -> SourcesResult {
        match self.service.clone().unwrap().run() {
            Ok(result) => Ok(SourcesOk::DiscoverSources(result)),
            Err(err) => Err(SourcesError::DiscoverSources(DiscoverSourcesError::Infra(
                err,
            ))),
        }
    }
}

pub enum RightSourcesQueries {
    DiscoverFiles(DiscoverSources),
}

pub enum SourcesQueries {
    Right(RightSourcesQueries),
    Left(),
}

pub enum SourcesCommands {
    Left(),
    Right(),
}

pub struct SourcesQueryFactory {}
impl SourcesQueryFactory {
    pub fn discover_files(path: String) -> SourcesQueries {
        let query = DiscoverSources::new(path);
        SourcesQueries::Right(RightSourcesQueries::DiscoverFiles(query))
    }
}

// pub struct SourcesQueriesFactory {}
// impl SourcesQueriesFactory {
//     pub fn discover_files(path: String) -> SourcesQueries {
//         let query = ReadSources::new(path);

//         SourcesQueries::ReadSources(query)
//     }
// }
