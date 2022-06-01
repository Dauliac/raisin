use super::UuidDTO;
use crate::domain::sources::file::File;
use crate::domain::sources::sources::Sources;
use crate::{core::domain::Entity, domain::program::Language};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use uuid::Uuid;

pub type PathDTO = String;

#[derive(Debug, Clone)]
pub struct FileDTO {
    pub uuid: UuidDTO,
    pub path: PathDTO,
    pub language: Language,
    pub lines: HashMap<u64, String>,
    pub includes: HashSet<UuidDTO>,
}

#[derive(Debug, Clone)]
pub struct SourcesDTO {
    pub uuid: UuidDTO,
    pub files: HashMap<UuidDTO, FileDTO>,
    pub language: Language,
}

impl From<&File> for FileDTO {
    fn from(file: &File) -> Self {
        let mut lines = HashMap::new();
        lines.extend(
            file.get_lines()
                .map(|(line, code)| (line.clone(), code.clone())),
        );

        let mut includes = HashSet::new();
        for uuid in file.get_includes() {
            let file: UuidDTO = uuid.to_string();
            includes.insert(file);
        }

        FileDTO {
            uuid: file.get_uuid().to_string(),
            path: file.get_path().to_str().unwrap().to_string(),
            language: file.get_language(),
            lines,
            includes,
        }
    }
}

impl Into<File> for FileDTO {
    fn into(self) -> File {
        let lines = self.lines.clone();

        let uuid = Uuid::parse_str(self.uuid.as_str()).ok().unwrap();
        let mut file = File::new(
            Some(uuid),
            Path::new(self.path.as_str()).to_path_buf(),
            self.language.clone(),
        );

        for uuid in self.includes.iter() {
            // TODO add pattern matching ?
            let include: Uuid = Uuid::parse_str(uuid.as_str()).ok().unwrap();
            file.include(include);
        }

        for (number, line) in self.lines.iter() {
            let line = line.clone();
            file.insert_line(*number, line);
        }

        file
    }
}

impl From<Sources> for SourcesDTO {
    fn from(sources: Sources) -> Self {
        let dot_files = HashMap::new();
        // let files = sources.get_files().copied();
        for (_uuid, file) in sources.get_files() {
            let file_dto = FileDTO::from(file);
        }
        SourcesDTO {
            uuid: sources.get_uuid().to_string(),
            files: dot_files,
            language: sources.get_language(),
        }
    }
}

impl Into<Sources> for SourcesDTO {
    fn into(self) -> Sources {
        let uuid = Uuid::parse_str(self.uuid.as_str()).ok();
        let mut sources = Sources::new(uuid, self.language.clone());
        for (_uuid, file_dto) in self.files.iter() {
            let file: File = file_dto.clone().into();
            sources.add_file(file)
        }

        sources
    }
}
