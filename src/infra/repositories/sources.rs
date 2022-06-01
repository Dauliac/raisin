use std::rc::Rc;
use uuid::Uuid;

use crate::app::cqrs::queries::sources::RightSourcesQueries;
use crate::app::cqrs::queries::sources::SourcesCommands;
use crate::app::cqrs::queries::sources::SourcesOk;
use crate::app::cqrs::queries::sources::SourcesQueries;
use crate::app::cqrs::queries::sources::SourcesResult;
use crate::app::cqrs::Query;
use crate::app::mappers;
use crate::core::domain::Entity;
use crate::domain::program::Language;
use crate::domain::repository::Repository;
use crate::domain::sources::sources::Sources;
use crate::infra::services::sources::source_reader::SourceReader;

pub struct Config {
    pub service: Rc<SourceReader>,
    pub path: String,
    pub language: Language,
}

pub struct SourcesRepository {
    uuid: Uuid,
    aggregate: Sources,
    config: Config,
}

impl Entity for SourcesRepository {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl SourcesRepository {
    pub fn new(config: Config) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            aggregate: Sources::new(None, config.language.clone()),
            config,
        }
    }
    // fn left_right(&self, query: &SourcesQueries) -> SourcesResult {
    //     Ok(SourcesOk::DiscoverSources()
    // }
    fn right_read(&mut self, query: RightSourcesQueries) -> SourcesResult {
        match query {
            RightSourcesQueries::DiscoverFiles(mut query) => {
                query.service = Some(self.config.service.clone());
                let query_result = query.run();
                match query_result {
                    Ok(result) => match result {
                        SourcesOk::DiscoverSources(sources) => {
                            let sources_dto = sources.clone();
                            self.aggregate = sources.into();
                            Ok(SourcesOk::DiscoverSources(sources_dto))
                        }
                        _ => Ok(result),
                    },
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl Repository<SourcesCommands, SourcesQueries, SourcesResult> for SourcesRepository {
    fn read(&mut self, query: SourcesQueries) -> SourcesResult {
        let result = match query {
            SourcesQueries::Right(query) => self.right_read(query),
            SourcesQueries::Left() => Ok(SourcesOk::Nothing),
        };
        return result;
    }

    fn write(&mut self, query: SourcesCommands) {}
}
