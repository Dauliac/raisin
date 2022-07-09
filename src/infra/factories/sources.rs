use std::path::PathBuf;
use async_trait::async_trait;

use crate::core::domain::Aggregate;
use crate::domain::program::Language;
use crate::domain::sources::aggregate::Sources;
use crate::domain::sources::factory::SourcesFactory;
use crate::infra::services::sources::source_discover::{
    SourceDiscover,
    Config as SourceDiscoverConfig,

};
use crate::infra::services::sources::source_discover:: Error as SourceDiscoverErrors;

pub struct Config {
    pub path: PathBuf,
    pub language: Language,
}

pub struct DefaultSourcesFactory {
    service: SourceDiscover,
}

impl DefaultSourcesFactory {
    pub fn new(config: Config) -> Self {
        let service_config = SourceDiscoverConfig {
            path: config.path,
            language: config.language,
        };
        Self {
            service: SourceDiscover::new(service_config),
        }
    }
}

#[async_trait]
impl SourcesFactory<SourceDiscoverErrors, <Sources as Aggregate>::Event> for DefaultSourcesFactory {
    async fn register(
        &self,
    ) -> Result<(Sources, Vec<<Sources as Aggregate>::Event>), SourceDiscoverErrors> {
        self.service.run().await
    }
}
