use std::sync::Arc;
use tree_sitter::Language as TsLanguage;
use tree_sitter::Node;
use tree_sitter::Parser as TsParser;

use crate::core::domain::{Entity, Uuid};
use crate::domain::cfg::aggregate::Cfg;
use crate::domain::cfg::aggregate::CfgEvent;
use crate::domain::program::Language;
// use crate::domain::program::Languages;
use crate::domain::sources::aggregate::Sources;
use crate::domain::sources::code::Code;
use crate::domain::sources::code::Coordinate;
use crate::domain::sources::code::Point;
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
    fn run(&self, sources: Arc<Sources>) -> Result<(Vec<Cfg>, Vec<CfgEvent>), Error> {
        let mut cfgs = Vec::new();
        let mut events = Vec::new();

        // #[cfg(any(unix, target_os = "linux"))]
        for file in sources.get_files() {
            let file = file.1;
            let text = file.get_text();
            let language = self.detect_language(&file.get_language());
            let mut parser = TsParser::new();
            // TODO check if bad language is an error ?
            // maybe yes ?
            parser.set_language(language).unwrap();

            match parse(text, &mut parser, file.get_uuid()) {
                Ok((mut new_cfgs, mut new_events)) => {
                    cfgs.append(&mut new_cfgs);
                    events.append(&mut new_events);
                }
                Err(error) => return Err(error),
            };
        }

        Ok((cfgs, events))
    }
}

fn parse(
    text: String,
    parser: &mut TsParser,
    file_uuid: Uuid,
) -> Result<(Vec<Cfg>, Vec<CfgEvent>), Error> {
    let tree = parser.parse(text.clone(), None).unwrap();
    let bytes = text.as_bytes();
    let mut cursor = tree.root_node().walk();
    let mut reached_root = false;
    let mut cfgs: Vec<Cfg> = Vec::new();
    let mut events: Vec<CfgEvent> = Vec::new();

    while reached_root == false {
        if cursor.node().is_named() {
            let slice = &bytes[cursor.node().range().start_byte..cursor.node().range().end_byte];
            let content = std::str::from_utf8(slice).unwrap();

            if cursor.node().kind() == "function_item" {
                let (cfg, mut new_events) = parse_cfg(cursor.node(), bytes, file_uuid.clone());
                cfgs.push(cfg);
                events.append(&mut new_events);
            }
        }

        if cursor.goto_first_child() {
            continue;
        } else {
            if cursor.goto_next_sibling() {
                continue;
            }
        }

        let mut retracing = true;
        while retracing {
            if !cursor.goto_parent() {
                retracing = false;
                reached_root = true;
            } else {
            }
            if cursor.goto_next_sibling() {
                retracing = false;
            }
        }
        if reached_root {
            break;
        }
    }

    Ok((cfgs, events))
}

fn parse_cfg(node: Node, bytes: &[u8], file_uuid: Uuid) -> (Cfg, Vec<CfgEvent>) {
    let slice = &bytes[node.range().start_byte..node.range().end_byte];
    // TODO: Should we add content into events
    // let content = std::str::from_utf8(slice).unwrap();
    let start_position = node.start_position();
    let end_position = node.end_position();
    let start = Point {
        row: start_position.row,
        column: start_position.column,
    };
    let end = Point {
        row: end_position.row,
        column: end_position.column,
    };
    let coordinate = Coordinate { start, end };
    let code = Code::new(coordinate, file_uuid);
    let (mut cfg, mut events) = Cfg::discover(code);

    let cfg_nodes_to_parse = node
        .descendant_for_byte_range(node.range().start_byte, node.range().end_byte)
        .unwrap();

    // parse_block(cfg_nodes, bytes, &mut cfg);
    return (cfg, events);
}

// fn parse_block(node: Node, bytes: &[u8], cfg: &mut Cfg) -> Result<Vec<CfgEvent>, Error> {
//     Ok()
// }
