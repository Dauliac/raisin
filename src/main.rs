pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use infra::services::sources::source_reader::{Config, SourceReader};

fn main() {
    let program = domain::program::Program::new();
    let config = Config {
        path: "./src".to_string(),
    };
    let mut service = SourceReader::new(config);
    let files = match service.run() {
        Ok(files) => println!("files:\n {:?}", files),
        Err(_) => return,
    };
}
