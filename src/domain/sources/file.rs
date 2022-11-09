use crate::core::domain::{new_uuid, Entity, Uuid};
use crate::domain::languages::Languages;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Iter as MapIter;
use std::collections::hash_set::Iter as HashSetIter;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct FileUuid(Uuid);
impl FileUuid {
    pub fn new() -> Self {
        Self(new_uuid())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct File {
    uuid: FileUuid,
    pub path: PathBuf,
    lines: BTreeMap<usize, String>,
    language: Languages,
    includes: HashSet<FileUuid>,
}

impl Entity<Self> for File {
    type Uuid = FileUuid;
    fn uuid(&self) -> FileUuid {
        self.uuid.clone()
    }
    fn equals(&self, entity: Box<Self>) -> bool {
        self.uuid == entity.uuid()
    }
}

impl File {
    pub fn new(uuid: FileUuid, path: PathBuf, language: Languages) -> File {
        Self {
            uuid,
            path,
            language,
            lines: BTreeMap::new(),
            includes: HashSet::new(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn insert_line(&mut self, line_number: usize, line: String) {
        self.lines.insert(line_number, line);
    }

    pub fn lines(&self) -> MapIter<usize, String> {
        self.lines.iter()
    }

    pub fn text(&self) -> String {
        self.lines().fold("".to_owned(), |mut text, line| {
            let (_, line) = line;
            text += line.as_str();
            text += "\n";
            text
        })
    }

    pub fn include(&mut self, uuid: FileUuid) {
        self.includes.insert(uuid);
    }

    pub fn includes(&self) -> HashSetIter<FileUuid> {
        self.includes.iter()
    }

    pub fn language(&self) -> Languages {
        self.language.clone()
    }
    // pub fn insert_content(&mut self, code: Code) {
    //     self.lines.insert(code.coordinate.start_line, code);
    // }
}
