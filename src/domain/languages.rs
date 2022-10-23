use serde::{Deserialize, Serialize};

pub trait Language {
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
impl Language for Cpp {
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
impl Language for Rust {
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
impl Language for C {
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
impl Language for Java {
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
pub enum Languages {
    Cpp(Cpp),
    Rust(Rust),
    C(C),
    Java(Java),
}

pub struct AvailableLanguages {}
impl AvailableLanguages {
    pub fn rust() -> Languages {
        let lang = Rust {};

        Languages::Rust(lang)
    }

    pub fn cpp() -> Languages {
        let lang = Cpp {};
        Languages::Cpp(lang)
    }
}
