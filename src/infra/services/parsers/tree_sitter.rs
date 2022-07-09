use std::fs::File as FsFile;
use std::sync::Arc;
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
    fn run(&self, sources: Arc<Sources>) -> Result<Vec<Cfg>, Error> {
        let cfgs = Vec::new();

        // #[cfg(any(unix, target_os = "linux"))]
        for file in sources.get_files() {
            let file = file.1;
            let text = file.get_text();
            let language = self.detect_language(&file.get_language());
            let mut parser = TsParser::new();
            // TODO check if bad language is an error ?
            // maybe yes ?
            parser.set_language(language).unwrap();

            parse(text, &mut parser);
        }

        Ok(cfgs)
    }
}

fn parse(text: String, parser: &mut TsParser) {
    let tree = parser.parse(text, None).unwrap();
    let mut tree = tree.walk();
    let node = tree.node();
    tree.goto_first_child();
    let node = tree.node();
    println!("{:?}", &node);
    tree.goto_next_sibling();
    let node = tree.node();
    println!("{:?}", &node);
    tree.goto_first_child();
    let node = tree.node();
    println!("{:?}", &node);
    tree.goto_next_sibling();
    let node = tree.node();
    println!("{:?}", &node);
    tree.goto_first_child();
}
