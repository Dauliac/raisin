use std::path::Path;

use crate::app::dtos::sources::Path as PathDTO;
use crate::domain::sources::sources::Error;

use walkdir::WalkDir;

pub struct Config {
    pub path: String,
}

pub struct SourceReader {
    config: Config,
}

impl SourceReader {
    pub fn new(conf: Config) -> SourceReader {
        SourceReader { config: conf }
    }

    pub fn run(&self) -> Result<Vec<PathDTO>, Error> {
        let mut files = Vec::new();
        let mut failed_files = Vec::new();

        let path = Path::new(self.config.path.as_str());
        if !path.exists() {
            return Err(Error::NotExists(path.to_owned()));
        }
        for file in WalkDir::new(self.config.path.as_str())
            .into_iter()
            .filter_map(|file| file.ok())
        {
            let metadata = file.metadata();

            if metadata.is_err() {
                failed_files.push(file.into_path().to_str().unwrap().to_string());
                continue;
            }
            let metadata = metadata.unwrap();
            if metadata.is_file() {
                files.push(file.into_path().to_str().unwrap().to_string());
                continue;
            } else if metadata.is_dir() {
                continue;
            } else {
                files.push(file.into_path().to_str().unwrap().to_string());
            }
        }

        if path.is_dir() && files.is_empty() {
            return Err(Error::EmptyDir(path.to_owned()));
        }

        if !failed_files.is_empty() {
            return Err(Error::FailedToIndexFiles {
                files: files.to_owned(),
                failed_files: failed_files.to_owned(),
            });
        }
        return Ok(files.to_owned());
    }
}

// pub struct CodeReaderConfig {
//     pub start_line: u64,
//     pub end_line: u64,
//     pub start_char: u64,
//     pub end_char: u64,
//     pub file: File,
// }

// pub struct CodeReader {
//     pub config: CodeReaderConfig,
//     pub files: HashMap<Uuid, File>,
// }

// impl CodeReader {
//     pub fn new(config: CodeReaderConfig) -> Self {
//         Self {
//             config,
//             files: HashMap::new(),
//         }
//     }

//     // pub fn run(&mut self) -> Result<Code, SourcesError> {
//     //     Ok(Code::new());
//     // }
// }
