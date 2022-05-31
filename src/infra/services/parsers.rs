use std::collections::HashMap;
use thiserror::Error;

use self::tree_sitter::TreeSitterParserService;

use crate::app::dtos::cfg::Cfg as CfgDTO;
use crate::app::dtos::sources::File as FileDTO;
use crate::app::dtos::Uuid as UuidDTO;

pub mod scope;
pub mod tree_sitter;

pub trait Parser {
    fn run(&self, file: &FileDTO) -> Result<HashMap<UuidDTO, CfgDTO>, Error>;
}

pub struct AvailableParsers {}
impl AvailableParsers {
    pub fn tree_sitter() -> Box<dyn Parser> {
        return Box::new(TreeSitterParserService::new());
    }

    // pub fn clang() -> Box<dyn Parser> {
    //     return Box::new(TreeSitterParserService::new());
    // }
}

#[derive(Error, Debug)]
pub enum Error {}
