use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;
use uuid::Uuid;

use crate::app::cqrs::Query;
use crate::app::dtos::cfg::Cfg as CfgDTO;
use crate::app::dtos::sources::File as FileDTO;
use crate::app::dtos::Uuid as UuidDTO;
use crate::core::domain::Entity;
use crate::infra::services::parsers::Error as InfraParserError;
use crate::infra::services::parsers::Parser;

// Right side Query
#[derive(Error, Debug)]
pub enum ParseFileError {
    #[error("Infrastructure error")]
    Infra(InfraParserError),
    #[error("Domain error")]
    Domain,
}

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("Failed to parse file")]
    ParseFile(ParseFileError),
}

#[derive(Debug)]
pub enum CfgOk {
    ParseFile(HashMap<UuidDTO, CfgDTO>),
}

pub type CfgResult = Result<CfgOk, CfgError>;

pub struct ParseFile {
    uuid: Uuid,
    file: FileDTO,
    pub service: Option<Rc<dyn Parser>>,
    // TOTO: check if we need app errors: NotRunned ?
    // result: Option<Result<HashMap<UuidDTO, CfgDTO>, ParseFileError>>,
}

impl Entity for ParseFile {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl ParseFile {
    pub fn new(file: FileDTO) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            file,
            service: None,
        }
    }
}

impl Query<CfgResult> for ParseFile {
    fn run(&mut self) -> CfgResult {
        match self.service.unwrap().run(&self.file) {
            Ok(result) => Ok(CfgOk::ParseFile(result)),
            Err(err) => Err(CfgError::ParseFile(ParseFileError::Infra(err))),
        }
        // self.result = Some(res);

        // self.result.unwrap()
    }
}

pub enum RightCfgCommands {
    DiscoverFiles(ParseFile),
}

pub struct CfgQueryFactory {}
impl CfgQueryFactory {
    pub fn parse_files(file: FileDTO) -> RightCfgCommands {
        let query = ParseFile::new(file);
        RightCfgCommands::DiscoverFiles(query)
    }
}
