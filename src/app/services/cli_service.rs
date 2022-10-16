use clap::{Parser, ArgEnum, Subcommand};
use clap_complete::Shell;
use async_trait::async_trait;

use super::Service;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Language {
    Cpp,
    Rust,
    C,
    Java,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about = "Code properties grape", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Parse sources
    Parse {
        #[clap(value_parser, value_hint = clap::ValueHint::AnyPath)]
        path: std::path::PathBuf,
        #[clap(short, long, arg_enum, value_parser)]
        language: Language,
    },
    /// Shell completion generator
    Complete {
        #[clap(arg_enum, value_parser)]
        shell: Shell,
    }
}

pub struct ArgParserService  {
}

impl ArgParserService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Service<Cli> for ArgParserService {
    async fn run(&mut self) -> Cli {
        let options = Cli::parse();
        println!("{:?}", options);
        options
    }
}

