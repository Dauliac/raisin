use std::path::PathBuf;
use async_trait::async_trait;

use crate::core::domain::Aggregate;
use crate::domain::program::Language;
use crate::domain::sources::aggregate::Sources;
use crate::domain::sources::factory::SourcesFactory;
use crate::infra::services::sources::source_reader::{
    Config as SourceReaderConfig, Error as SourceReaderErrors, SourceReader,
};

pub struct Config {
    pub path: PathBuf,
    pub language: Language,
}

pub struct DefaultSourcesFactory {
    service: SourceReader,
}

impl DefaultSourcesFactory {
    pub fn new(config: Config) -> Self {
        let service_config = SourceReaderConfig {
            path: config.path,
            language: config.language,
        };
        Self {
            service: SourceReader::new(service_config),
        }
    }
}

#[async_trait]
impl SourcesFactory<SourceReaderErrors, <Sources as Aggregate>::Event> for DefaultSourcesFactory {
    async fn register(
        &self,
    ) -> Result<(Sources, Vec<<Sources as Aggregate>::Event>), SourceReaderErrors> {
        self.service.run().await
    }
}
