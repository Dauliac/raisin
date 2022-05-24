use std::path::PathBuf;
use uuid::Uuid;

use crate::app::cqrs::Command;
use crate::app::dtos::sources::Path as PathDTO;
use crate::core::domain::Entity;
use crate::domain::sources::sources::Error;

pub struct FilesWereDiscovered<F>
where
    F: Fn() -> Result<Vec<PathDTO>, Error>,
{
    uuid: Uuid,
    pub path: PathBuf,
    // TODO: dauliac add Result into commands
    pub result: Option<Result<Vec<PathDTO>, Error>>,
    callback: F,
}

impl<F> Entity for FilesWereDiscovered<F>
where
    F: Fn() -> Result<Vec<PathDTO>, Error>,
{
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl<F> Command for FilesWereDiscovered<F>
where
    F: Fn() -> Result<Vec<PathDTO>, Error>,
{
    fn run(&mut self) {
        self.result = Some((self.callback)());
    }
}

impl<F> FilesWereDiscovered<F>
where
    F: Fn() -> Result<Vec<PathDTO>, Error>,
{
    pub fn new(path: PathBuf, callback: F) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            path,
            callback,
            result: None,
        }
    }
}
