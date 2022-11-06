use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};
use std::path::PathBuf;
use thiserror::Error;

use crate::core::domain::{new_uuid, Aggregate, Entity, Uuid, Event, Value};
use super::cfg::aggregate::{CfgError, CfgUuid};
use super::languages::Languages;
use super::sources::aggregate::{
    DiscoverSources, Sources, SourcesError, SourcesEvent, SourcesUuid,
};
use super::cfg::aggregate::Cfg;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ProgramUuid(Uuid);
impl ProgramUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

impl Value<ProgramUuid> for ProgramUuid {
    fn equals(&self, value: &ProgramUuid) -> bool {
        self == value
    }
}

#[derive(EnumString, EnumVariantNames, IntoStaticStr, Default, Error, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum ProgramError {
    #[error("Nothing")]
    #[default]
    Unknown,
    #[error("No sources with uuid {0} found")]
    SourcesNotFound(SourcesUuid),
    #[error("No cfg with uuid {0} found")]
    CfgNotFound(CfgUuid),
    #[error("Sources error")]
    Sources(SourcesError),
    #[error("Cfg error")]
    Cfg(CfgError),
}

impl Event<ProgramError> for ProgramError {
    fn get_id(&self) -> String {
        match &self {
            ProgramError::Unknown => {
                String::new()
            },
            ProgramError::CfgNotFound(..) | ProgramError::SourcesNotFound(..) => {
                let id: &'static str = self.into();
                format!("{}{}", Self::SEPARATOR, id)
            },
            ProgramError::Sources(error) => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, error.get_id().as_str())
            },
            ProgramError::Cfg(error) => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, error.get_id().as_str())
            },
        }
    }
}


#[derive(Default, Debug, EnumString, EnumVariantNames, IntoStaticStr, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ProgramEvent {
    #[default]
    Unknown,
    ProgramDiscovered {
        program_uuid: ProgramUuid,
        language: Languages,
        path: PathBuf,
    },
    Sources {
        program_uuid: ProgramUuid,
        event: SourcesEvent,
    },
    Cfg {
        program_uuid: ProgramUuid,
        event: <Cfg as Aggregate<Cfg>>::Event,
    },
}

impl Event<ProgramEvent> for ProgramEvent {
    fn get_id(&self) -> String {
        match &self {
            ProgramEvent::Unknown => {
                String::new()
            },
            ProgramEvent::ProgramDiscovered { .. } => {
                let id: &'static str = self.into();
                format!("{}{}", Self::SEPARATOR, id)
            },
            ProgramEvent::Sources {
              program_uuid: _,
              event
            } => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, event.get_id().as_str())
            },
            ProgramEvent::Cfg {
              program_uuid: _,
              event
            } => {
                let id: &'static str = self.into();
                format!("{}{}{}", Self::SEPARATOR, id, event.get_id().as_str())
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct DiscoverProgram {
    pub language: Languages,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ProgramCommand {
    DiscoverProgram(DiscoverProgram),
    Sources(<Sources as Aggregate<Sources>>::Command),
    Cfg(<Cfg as Aggregate<Cfg>>::Command),
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    uuid: ProgramUuid,
    cfgs: HashMap<CfgUuid, Cfg>,
    sources: Option<Sources>,
    path: PathBuf,
    language: Languages,
}

impl Entity<Self> for Program {
    type Uuid = ProgramUuid;
    fn get_uuid(&self) -> ProgramUuid {
        self.uuid.clone()
    }
    fn equals(&self, entity: Box<Program>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

#[async_trait]
impl Aggregate<Self> for Program {
    type Error = ProgramError;
    type Event = ProgramEvent;
    type Command = ProgramCommand;
    type Result = Result<Vec<Self::Event>, Self::Error>;

    async fn handle(&self, command: Self::Command) -> Self::Result {
        let mut events: Vec<Self::Event> = Vec::new();

        match command {
            Self::Command::DiscoverProgram(command) => {
                let event = Self::Event::ProgramDiscovered {
                    program_uuid: self.uuid.clone(),
                    language: command.language,
                    path: command.path,
                };
                events.push(event);
            }
            Self::Command::Sources(command) => {}
            Self::Command::Cfg(command) => {}
        };

        Ok(events)
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::Unknown => (),
            Self::Event::ProgramDiscovered {
                program_uuid: _,
                language,
                path,
            } => {
                self.language = language;
                self.path = path;
            }
            Self::Event::Sources {
                program_uuid: _,
                event,
            } => {
                match event {
                    SourcesEvent::Unknown => {}
                    SourcesEvent::SourcesDiscovered {
                        sources_uuid,
                        language,
                        path,
                    } => {}
                    SourcesEvent::FileIndexed { file_uuid, path } => {}
                    SourcesEvent::FileNotIndexed {} => {}
                    SourcesEvent::FileContentLoaded { file_uuid, code } => {}
                };
                // match self.sources.get_key_value()
            }
            Self::Event::Cfg {
                program_uuid,
                event,
            } => {}
        };
    }
}

impl Program {
    pub fn discover(command: DiscoverProgram) -> (Self, <Self as Aggregate<Self>>::Result) {
        let uuid = ProgramUuid::new();
        let mut events = vec![ProgramEvent::ProgramDiscovered {
            program_uuid: uuid.clone(),
            language: command.language.clone(),
            path: command.path.clone(),
        }];
        let sources_command = DiscoverSources {
            language: command.language.clone(),
            path: command.path.clone(),
        };

        let (sources, sources_result) = Sources::discover(sources_command);
        match sources_result {
            Ok(sources_events) => {
                for sources_event in sources_events {
                    let program_event = ProgramEvent::Sources {
                        program_uuid: uuid.clone(),
                        event: sources_event,
                    };
                    events.push(program_event);
                }
            }
            Err(error) => {
                return (
                    Self {
                        uuid: ProgramUuid::new(),
                        cfgs: HashMap::new(),
                        sources: None,
                        path: command.path,
                        language: command.language,
                    },
                    Err(ProgramError::Sources(error)),
                );
            }
        };

        return (
            Self {
                uuid: ProgramUuid::new(),
                cfgs: HashMap::new(),
                sources: Some(sources),
                path: command.path,
                language: command.language,
            },
            Ok(events),
        );
    }
}
