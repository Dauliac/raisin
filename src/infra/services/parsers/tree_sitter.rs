use std::fs::File as FsFile;
use tree_sitter::Language as TsLanguage;
use tree_sitter::Parser as TsParser;

use crate::domain::cfg::aggregate::Cfg;
use crate::domain::program::Language;
// use crate::domain::program::Languages;
use crate::domain::sources::aggregate::Sources;
// use crate::domain::sources::file::File;

use super::Error;
use super::Parser;

extern "C" {
    fn tree_sitter_rust() -> TsLanguage;
}

pub struct TreeSitterParserService {}

impl TreeSitterParserService {
    pub fn new() -> Self {
        Self {}
    }

    fn detect_language(&self, language: &Language) -> TsLanguage {
        match language {
            // TODO(dauliac): fix parsers with other tree_sitter parsers
            Language::Cpp(_) => unsafe { tree_sitter_rust() },
            Language::Rust(_) => unsafe { tree_sitter_rust() },
            Language::C(_) => unsafe { tree_sitter_rust() },
            Language::Java(_) => unsafe { tree_sitter_rust() },
        }
    }
    fn iter_on_nodes() {}
}

impl Parser for TreeSitterParserService {
    fn run(&self, file: &Sources) -> Result<Vec<Cfg>, Error> {
        let cfgs = Vec::new();
        let language = self.detect_language(&file.get_language());
        let mut parser = TsParser::new();
        // TODO check if bad language is an error ?
        // maybe yes ?
        parser.set_language(language).unwrap();

        // TODO remove this debug block
        let debug = FsFile::open("debug.log").unwrap();
        // #[cfg(any(unix, target_os = "linux"))]
        parser.print_dot_graphs(&debug);
        parser.stop_printing_dot_graphs();

        Ok(cfgs)
    }
}

// pub fn parse(language: &Languages) -> Result<>

// pub fn init() {
//     let source_code = "
// impl SourceReader {
//     pub fn run(&self) -> Result<SourcesDTO, Error> {
//         let mut files = HashMap::new();
//         let mut failed_files = HashMap::new();
//         {}

//         let path = Path::new(self.config.path.as_str());
//         if !path.exists() {
//             return Err(Error::NotExists(path.to_str().unwrap().to_string()));
//         }
//         for file in WalkDir::new(self.config.path.as_str())
//             .into_iter()
//             .filter_map(|file| file.ok())
//         {
//             let metadata = file.metadata();
//             let path = path.to_str().unwrap().to_string();
//             let file = FileDTO {
//                 uuid: Uuid::new_v4().to_string(),
//                 path,
//                 lines: HashMap::new(),
//                 includes: Vec::new(),
//             };

//             if metadata.is_err() {
//                 failed_files.insert(file.uuid.clone(), file);
//                 continue;
//             }

//             let metadata = metadata.unwrap();
//             if metadata.is_file() {
//                 files.insert(file.uuid.clone(), file);
//             }
//         }

//         if path.is_dir() && files.is_empty() {
//             return Err(Error::EmptyDir(path.to_str().unwrap().to_string()));
//         }

//         if !failed_files.is_empty() {
//             return Err(Error::FailedToIndexFiles {
//                 files,
//                 failed_files,
//             });
//         }

//         let indexed_files = HashMap::new();
//         let sources = SourcesDTO {
//             uuid: Uuid::new_v4().to_string(),
//             files: indexed_files,
//         };
//         return Ok(sources.to_owned());
//     }
// }
// ";
//     let tree = parser.parse(source_code, None).unwrap();
//     let mut tree = tree.walk();
//     let node = tree.node();
//     tree.goto_first_child();
//     let node = tree.node();
//     println!("{:?}", &node);
//     tree.goto_next_sibling();
//     let node = tree.node();
//     println!("{:?}", &node);
//     tree.goto_first_child();
//     let node = tree.node();
//     println!("{:?}", &node);
//     tree.goto_next_sibling();
//     let node = tree.node();
//     println!("{:?}", &node);
//     tree.goto_first_child();
// }
