use async_trait::async_trait;
use std::sync::Arc;

use crate::core::domain::Aggregate;
use crate::domain::cfg::aggregate::Cfg;
use crate::domain::sources::aggregate::Sources;
use crate::infra::services::parsers::Error;
use crate::infra::services::parsers::{AvailableParsers, Parser};

pub struct Config {
    pub sources: Arc<Sources>,
}

// pub struct DefaultCfgsFactory {
//     config: Config,
//     parser: Box<dyn Parser + 'static>,
// }

// impl DefaultCfgsFactory {
//     pub fn new(config: Config) -> Self {
//         let parser: Box<dyn Parser + 'static> = AvailableParsers::tree_sitter();
//         Self { config, parser }
//     }
// }

// #[async_trait]
// impl CfgsFactory<Error, <Cfg as Aggregate>::Event> for DefaultCfgsFactory {
//     fn register(&self) -> Result<(Vec<Cfg>, Vec<<Cfg as Aggregate>::Event>), Error> {
//         self.parser.run(self.config.sources.clone())
//     }
// }
