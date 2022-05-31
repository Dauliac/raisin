use crate::core::domain::Entity;
use crate::domain::cfg::cfg::Cfg;
use crate::domain::sources::file::File;
use std::boxed::Box;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Program {
    uuid: Uuid,
    cfgs: HashMap<String, Cfg>,
    sources: HashMap<String, File>,
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

pub trait Language {
    const NAME: &'static str;
    const IS_OBJECT_ORIENTED: bool;
    const IS_FUNCTIONAL_ORIENTED: bool;
    const HAS_HEADER: bool;
    const HAS_GENERICS: bool;
    const IS_BDD: bool;
    const EXTENTIONS: Vec<&'static str>;
}

#[derive(Debug, Clone)]
pub struct Cpp {}
impl Language for Cpp {
    const NAME: &'static str = "c++";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = true;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    const EXTENTIONS: Vec<&'static str> = vec!["hpp", "cpp", "h", "c++"];
}

#[derive(Debug, Clone)]
pub struct Rust;
impl Language for Rust {
    const NAME: &'static str = "rust";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = false;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    const EXTENTIONS: Vec<&'static str> = vec!["rs"];
}

#[derive(Debug, Clone)]
pub struct C;
impl Language for C {
    const NAME: &'static str = "c";
    const IS_OBJECT_ORIENTED: bool = false;
    const IS_FUNCTIONAL_ORIENTED: bool = true;
    const HAS_HEADER: bool = true;
    const HAS_GENERICS: bool = false;
    const IS_BDD: bool = false;
    const EXTENTIONS: Vec<&'static str> = vec!["c", "h"];
}

#[derive(Debug, Clone)]
pub struct Java;
impl Language for Java {
    const NAME: &'static str = "java";
    const IS_OBJECT_ORIENTED: bool = true;
    const IS_FUNCTIONAL_ORIENTED: bool = true; // Question
    const HAS_HEADER: bool = false;
    const HAS_GENERICS: bool = true;
    const IS_BDD: bool = false;
    const EXTENTIONS: Vec<&'static str> = vec!["java"];
}

#[derive(Debug, Clone)]
pub enum Languages {
    Cpp(Cpp),
    Rust(Rust),
    C(C),
    Java(Java),
}
