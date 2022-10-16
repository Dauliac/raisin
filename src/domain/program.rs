use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use thiserror::Error;

use super::sources::aggregate::Sources;
use crate::core::domain::{new_uuid, Aggregate, Entity, Uuid};
use crate::domain::cfg::aggregate::Cfg;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ProgramUuid(Uuid);
impl ProgramUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Sources error")]
    Sources(<Sources as Aggregate<Sources>>::Error),
    #[error("Cfg error")]
    Cfg(<Cfg as Aggregate<Cfg>>::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ProgramEvent {
    Sources(<Sources as Aggregate<Sources>>::Event),
    Cfg(<Cfg as Aggregate<Cfg>>::Event),
    ProgramDiscovered {
        program_uuid: ProgramUuid,
        language: Language,
    },
}

pub struct Program {
    uuid: ProgramUuid,
    cfgs: HashMap<<Cfg as Entity<Cfg>>::Uuid, Cfg>,
    sources: Option<Sources>,
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

impl Program {
    pub fn new() -> Self {
        Self {
            // uuid: ProgramUuid(new_uuid()),
            uuid: ProgramUuid::new(),
            cfgs: HashMap::new(),
            sources: None,
        }
    }
}

pub trait Languages {
    const NAME: &'static str;
    const IS_OBJECT_ORIENTED: bool;
    const IS_FUNCTIONAL_ORIENTED: bool;
    const HAS_HEADER: bool;
    const HAS_GENERICS: bool;
    const IS_BDD: bool;
    fn get_extentions(&self) -> Vec<&str>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Cpp {}
impl Languages for Cpp {
    const NAME: &'static str = "c++";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = true;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    fn get_extentions(&self) -> Vec<&str> {
        vec!["hpp", "cpp", "h", "c++"]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Rust;
impl Languages for Rust {
    const NAME: &'static str = "rust";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = false;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    fn get_extentions(&self) -> Vec<&str> {
        vec!["rs"]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct C;
impl Languages for C {
    const NAME: &'static str = "c";
    const IS_OBJECT_ORIENTED: bool = false;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = true;
    const HAS_GENERICS: bool = false;
    const IS_BDD: bool = false;
    fn get_extentions(&self) -> Vec<&str> {
        vec!["c", "h"]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Java;
impl Languages for Java {
    const NAME: &'static str = "java";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true; // Question
    const HAS_HEADER: bool = false;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    fn get_extentions(&self) -> Vec<&str> {
        vec!["java"]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Language {
    Cpp(Cpp),
    Rust(Rust),
    C(C),
    Java(Java),
}

pub struct AvailableLanguages {}
impl AvailableLanguages {
    pub fn rust() -> Language {
        let lang = Rust {};

        Language::Rust(lang)
    }

    pub fn cpp() -> Language {
        let lang = Cpp {};
        Language::Cpp(lang)
    }
}
