use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::app::cqrs::CommandHandler;
use crate::app::events::file::{Error, FilesWereDiscovered};
use crate::core::domain::Entity;
use crate::domain::repository::Repository;
use crate::domain::sources::file::File;
use crate::infra::services::sources::source_reader::SourceReader;

pub struct Config {
    pub service: SourceReader,
    pub path: PathBuf,
    pub command_handler: Box<dyn CommandHandler>,
}

pub struct FileDiscoverRepository {
    uuid: Uuid,
    aggregate: Sources,
    config: Config,
}

impl Entity for FileDiscoverRepository {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl FileDiscoverRepository {
    fn new(config: Config) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            aggregate: Sources::new(),
            config,
        }
    }
}

impl Repository<Sources, Error> for FileDiscoverRepository {
    fn read(&mut self) -> Result<File, Error> {
        // Put service into command
        let command = Box::new(FilesWereDiscovered::new(self.config.path, || {
            // self.config.service.run()
            match self.config.service.run() {
                Err(err) => Err(Error::FailedToIndex(err)),
                Ok(files) => Ok(files),
            }
        }));

        // Execute command
        self.config.command_handler.handle(command);

        let paths = command.result?;
        for path in paths.iter() {
            this.aggregate.index_sources(path);
        }
        return Ok(this.aggregate);
    }
}
