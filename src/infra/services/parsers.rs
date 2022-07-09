use std::sync::Arc;

use thiserror::Error;

use crate::domain::{cfg::aggregate::Cfg, sources::aggregate::Sources};

use self::tree_sitter::TreeSitterParserService;

pub mod scope;
pub mod tree_sitter;

pub trait Parser {
    fn run(&self, sources: Arc<Sources>) -> Result<Vec<Cfg>, Error>;
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
