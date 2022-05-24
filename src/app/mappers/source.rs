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

impl From<UuidDTO> for Uuid {
    fn from(uuid: UuidDTO) -> Self {
        Uuid::parse_str(uuid)
    }
}

impl From<Path> for PathDTO {
    fn from(path: Path) -> Self {
        path.to_str().to_string()
    }
}

impl From<PathDTO> for Path {
    fn from(path: PathDTO) -> Self {
        Path::new(path.as_str())
    }
}

impl From<File> for FileDTO {
    fn from(file: File) -> Self {
        FileDTO {
            uuid: UuidDTO::from(file.get_uuid()),
        }
    }
}

impl From<FileDTO> for File {
    fn from(file: FileDTO) -> Self {
        Sources {}
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

impl From<SourcesDTO> for Sources {
    fn from(sources: SourcesDTO) -> Self {
        Sources {}
    }
}
