use std::rc::Rc;
use uuid::Uuid;

use crate::app::cqrs::commands::file::SourcesCommand;
use crate::app::cqrs::queries::file::SourcesQueries;
use crate::app::cqrs::CommandHandler;
use crate::core::domain::Entity;
use crate::domain::repository::Repository;
use crate::domain::sources::sources::{Error, Sources};
use crate::infra::services::sources::source_reader::SourceReader;

pub struct Config {
    pub service: Rc<SourceReader>,
    pub command_handler: Box<dyn CommandHandler>,
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
}

impl Repository<SourcesCommand, SourcesQueries, Error> for SourcesRepository {
    fn write(&mut self, command: SourcesCommand) -> Result<(), Error> {
        let command = match command {
            SourcesCommand::DiscoverFiles(mut command) => {
                command.service = Some(self.config.service.to_owned());
                command
            }
        };
        let command = Box::new(command);

        // Execute command
        self.config.command_handler.handle(command);

        // let paths = command.result?;
        // for path in paths.iter() {
        //     self.aggregate.index_sources(path);
        // }
        return Ok(());
    }
    fn read(&mut self, query: SourcesQueries) -> Result<(), Error> {
        return Ok(());
    }
}
