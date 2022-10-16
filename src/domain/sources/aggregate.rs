use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::{collections::{hash_map::Iter, HashMap}, path::PathBuf};
use std::hash::Hash;
use async_trait::async_trait;

use super::{file::{File, FileUuid}, code::Code};
use crate::core::domain::{new_uuid, Aggregate, Entity, Uuid, Event};
use crate::domain::program::Language;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct SourcesUuid(Uuid);
impl SourcesUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

#[derive(Error, Debug)]
pub enum SourcesError {
    #[error("given dir `${path:?}` not exists")]
    FileNotIndexed { path: PathBuf },
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum SourcesEvent {
    SourcesDiscovered {
        sources_uuid: SourcesUuid,
        language: Language,
        path: PathBuf,
    },
    FileIndexed {
        file_uuid: FileUuid,
        path: PathBuf,
    },
    FileNotIndexed {
    },
    FileContentLoaded {
        file_uuid: FileUuid,
        code: Code,
    },
}

#[derive(PartialEq, Eq, Hash)]
pub struct DiscoverSources {
    language: Language,
    path: PathBuf,
}

#[derive(PartialEq, Eq, Hash)]
pub enum SourcesCommand {
    DeclareSources(DiscoverSources),
    IndexFile {
        path: PathBuf,
    },
    LoadFileContent {
        file_uuid: FileUuid,
        code: Code
    },
}

impl Event for SourcesEvent {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    uuid: SourcesUuid,
    files: HashMap<FileUuid, File>,
    language: Language,
    path: PathBuf,
}

impl Entity<Self> for Sources {
    type Uuid = SourcesUuid;
    fn get_uuid(&self) -> SourcesUuid {
        self.uuid.clone()
    }

    fn equals(&self, entity: Box<Self>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

#[async_trait]
impl Aggregate<Self> for Sources {
    type Error = SourcesError;
    type Event = SourcesEvent;
    type Command = SourcesCommand;
    type Result = Result<Vec<Self::Event>, Self::Error>;

    async fn handle(&self, command: Self::Command) ->  Self::Result {
        let mut events = Vec::new();
        match command {
            Self::Command::DeclareSources(command) => {
                let event = Self::Event::SourcesDiscovered {
                    sources_uuid: self.uuid.clone(),
                    language: command.language,
                    path: command.path
                };
                events.push(event);
            },
            Self::Command::IndexFile {
              path,
            } => {
                let uuid = FileUuid::new();
                let event = Self::Event::FileIndexed {
                    file_uuid: uuid,
                    path,
                };
                events.push(event);
            },
            Self::Command::LoadFileContent {
              file_uuid,
              code,
            } => {
                let event = Self::Event::FileContentLoaded {
                    file_uuid: file_uuid.clone(),
                    code,
                };
                events.push(event);
            },
        }

        Ok(events)
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::SourcesDiscovered {
              sources_uuid: _,
              language,
              path,
            } => {
                self.files = HashMap::new();
                self.language = language;
                self.path = path;
            },
            Self::Event::FileIndexed {
              file_uuid,
              path,
            } => {
                let file = File::new(file_uuid.clone(), path, self.language.clone());
                self.files.insert(file_uuid, file);
            },
            Self::Event::FileNotIndexed {
            } => (),
            Self::Event::FileContentLoaded {
                file_uuid: _,
                code: _,
            } => {
                // TODO: Add file method to load code
                // self.code = code;
            },
        }
    }
}

impl Sources {
    fn new(uuid: SourcesUuid, language: Language, path: PathBuf) -> Self {
        Self {
            uuid,
            files: HashMap::new(),
            language,
            path,
        }
    }

    pub fn discover(command: DiscoverSources) -> (Self, <Self as Aggregate<Self>>::Result) {
        let uuid = SourcesUuid::new();
        let events = vec![SourcesEvent::SourcesDiscovered {
            sources_uuid: uuid.clone(),
            language: command.language.clone(),
            path: command.path.clone(),
        }];
        return (
            Self::new(uuid, command.language, command.path),
            Ok(events),
        )
    }

    pub fn get_files(&self) -> Iter<FileUuid, File> {
        self.files.iter()
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
}
