use std::rc::Rc;
use uuid::Uuid;

use crate::app::cqrs::queries::sources::RightSourcesQueries;
use crate::app::cqrs::queries::sources::SourcesCommands;
use crate::app::cqrs::queries::sources::SourcesOk;
use crate::app::cqrs::queries::sources::SourcesQueries;
use crate::app::cqrs::queries::sources::SourcesResult;
use crate::app::cqrs::Query;
use crate::core::domain::Entity;
use crate::domain::repository::Repository;
use crate::domain::sources::sources::Sources;
use crate::infra::services::sources::source_reader::SourceReader;

pub struct Config {
    pub service: Rc<SourceReader>,
    pub path: String,
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
            aggregate: Sources::new(),
            config,
        }
    }
    // fn left_right(&self, query: &SourcesQueries) -> SourcesResult {
    //     Ok(SourcesOk::DiscoverSources()
    // }
    fn right_write(&self, query: &RightSourcesQueries) -> SourcesResult {
        match query {
            RightSourcesQueries::DiscoverFiles(query) => {
                query.service = Some(self.config.service);
                query.to_owned().run()
            }
        }
    }
}

impl Repository<SourcesCommands, SourcesQueries, SourcesResult> for SourcesRepository {
    fn read(&mut self, query: SourcesQueries) -> SourcesResult {
        let result = match query {
            SourcesQueries::Right(query) => self.right_write(&query),
            SourcesQueries::Left() => Ok(SourcesOk::Nothing),
        };
        return result;
    }

    fn write(&mut self, query: SourcesCommands) {}
}
