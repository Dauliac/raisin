use crate::app::dto::sources::File as FileDTO;
use crate::app::dto::sources::Path as PathDTO;
use crate::app::dto::sources::Sources as SourcesDTO;
use crate::app::dto::sources::Uuid as UuidDTO;
use crate::core::domain::sources::File;
use crate::core::domain::sources::Sources;

// TODO move it into dedicated file
impl From<Uuid> for UuidDTO {
    fn from(uuid: Uuid) -> Self {
        uuid.hyphenated().to_string()
    }
}

impl Into<Uuid> for UuidDTO {
    fn into(self) -> Uuid {
        Uuid::parse_str(self)
    }
}

impl From<Path> for PathDTO {
    fn from(path: Path) -> Self {
        path.to_str().to_string()
    }
}

impl Into<Path> for PathDTO {
    fn into(self) -> Path {
        Path::new(self.as_str())
    }
}

impl From<File> for FileDTO {
    fn from(file: File) -> Self {
        let mut lines = Vec::new();
        lines.extend(
            file.get_lines()
                .map(|(line, code)| (line.clone(), code.clone())),
        );

        let includes = Vec::new();
        for (uuid) in file.includes.iter() {
            let file: UuidDTO = UuidDTO::from(uuid);
            sources.push(file);
        }

        FileDTO {
            uuid: UuidDTO::from(file.get_uuid()),
            path: file.get_path(),
            lines,
            includes,
        }
    }
}

impl Into<File> for FileDTO {
    fn into(self) -> File {
        let mut lines = Vec::new();
        lines.clone_from(self.lines);

        for (uuid) in self.includes.iter() {
            let file: Uuid = uuid.into();
            includes.push(file);
        }

        File {
            uuid: self.uuid,
            path: self.path,
            lines,
            includes,
        }
    }
}

impl From<Sources> for SourcesDTO {
    fn from(sources: Sources) -> Self {
        let files = HashMap::new();
        for (uuid, &file) in sources.iter() {
            let file = File::from(file);
        }
        SourcesDTO {}
    }
}

impl Into<Sources> for SourcesDTO {
    fn into(self) -> Sources {
        Sources {}
    }
}
