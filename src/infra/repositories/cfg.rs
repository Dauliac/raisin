use crate::app::cqrs::commands::file::SourcesCommand;
use crate::app::cqrs::queries::file::SourcesQueries;
use crate::app::cqrs::CommandHandler;
use crate::core::domain::{Entity, Uuid};
use crate::domain::repository::Repository;
use crate::domain::sources::sources::Sources;
use crate::infra::services::sources::source_reader::Error;
use crate::infra::services::sources::source_reader::SourceReader;

pub struct Config {
    pub path: String,
}

pub struct CfgRepository {
    uuid: Uuid,
    aggregates: HashMap<Uuid, Cfg>,
    config: Config,
}

impl Entity for CfgRepository {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl CfgRepository {
    pub fn new(config: Config) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            aggregate: HashMap::new(),
            config,
        }
    }
}

impl Repository<Cfg> for CfgRepository {
    fn read(&mut self, uuid: Uuid) -> Option<&Cfg> {
        return self.aggregates.get(uuid);
    }

    fn write(&mut self, command: CfgCommand) {
        return;
    }
}
