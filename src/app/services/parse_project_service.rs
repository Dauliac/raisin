use std::sync::Arc;
use async_trait::async_trait;

use crate::{infra::{event_bus::{EventBus, Event}, services::parsers::AvailableParsers, repositories::cfg::CfgRepository}, domain::{sources::aggregate::Sources, repository::Repository}};
use super::Service;

pub struct Config {
    pub sources: Arc<Sources>,
    pub event_bus: Arc<dyn EventBus + Send + Sync>,
}

pub struct ParseProjectService {
    config: Config,
}

impl ParseProjectService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Service<CfgRepository> for ParseProjectService {
    async fn run(&mut self) -> CfgRepository {
        let mut repo = CfgRepository::new();

        // TODO move it into factory
        let parse = AvailableParsers::tree_sitter();
        let sources = self.config.sources.clone();
        match parse.run(sources) {
            Ok((cfgs, events)) => {
                for event in events {
                    let event = Event::Cfg(event);
                    self.config.event_bus.publish(event);
                }
                for cfg in cfgs {
                    repo.write(cfg);
                }
            }
            Err(_) => ()
        }

        repo
    }
}
