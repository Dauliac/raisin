use super::Uuid;
use crate::domain::program::Languages;
use std::collections::HashMap;

pub type Path = String;

#[derive(Debug, Clone)]
pub struct File {
    pub uuid: Uuid,
    pub path: Path,
    pub language: Languages,
    pub lines: HashMap<u64, String>,
    pub includes: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct Sources {
    pub uuid: Uuid,
    pub files: HashMap<Uuid, File>,
}
