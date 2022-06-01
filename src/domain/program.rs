use crate::core::domain::Entity;
use crate::domain::cfg::cfg::Cfg;
use crate::domain::sources::file::File;
use std::boxed::Box;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Program {
    uuid: Uuid,
    cfgs: HashMap<Uuid, Cfg>,
    sources: HashMap<Uuid, File>,
}

impl Entity for Program {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            cfgs: HashMap::new(),
            sources: HashMap::new(),
        }
    }
}

enum Paradigms {
    Object,
    Functionnal,
    Header,
    Generics,
    Bdd,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
