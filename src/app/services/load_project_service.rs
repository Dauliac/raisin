use super::Service;
use crate::domain::program::Language;
use crate::infra::factories::sources::{Config as ConfigSourcesFactory, DefaultSourcesFactory};
use crate::infra::services::sources::source_reader::{
    SourceReader,
    Config as SourceReaderConfig,
};
use crate::{
    domain::{repository::Repository, sources::factory::SourcesFactory},
    infra::{
        event_bus::{Event, EventBus},
        repositories::sources::SourcesRepository,
    },
};

use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Config {
    pub path: PathBuf,
    pub language: Language,
    pub event_bus: Arc<dyn EventBus + Send + Sync>,
}

pub struct LoadProjectService {
    config: Config,
}

impl LoadProjectService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Service<SourcesRepository> for LoadProjectService {
    async fn run(&mut self) -> SourcesRepository {
        let mut repo = SourcesRepository::new();

        let config = ConfigSourcesFactory {
            path: self.config.path.clone(),
            language: self.config.language.clone(),
        };
        let sources_factory = DefaultSourcesFactory::new(config);
        let sources_result = sources_factory.register().await;

        if let Ok(result) = sources_result {
            let mut sources = result.0;
            let sources_creation_events = result.1;
            for event in sources_creation_events {
                let event = Event::Sources(event);

                let _ = self.config.event_bus.publish(event);
            }
            let sources_reader_config = SourceReaderConfig {
                sources: &mut sources,
            };
            let mut source_reader = SourceReader::new(sources_reader_config);
            match source_reader.run().await {
                Ok(events) => {
                    for event in events {
                        let event = Event::Sources(event);
                        let _ = self.config.event_bus.publish(event);
                    }
                },
                Err(_) => (),
            }

            repo.write(sources);
        }
        repo
    }
}
