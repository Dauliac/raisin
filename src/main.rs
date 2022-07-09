pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use std::path::PathBuf;
use app::services::Service;
use app::services::load_project_service::{LoadProjectService, Config as LoadProjectConfig };
use infra::event_bus::MemoryEventBus;
use tokio::main;
use std::sync::Arc;

use domain::program::AvailableLanguages;

#[main]
async fn main() {
    // let _program = domain::program::Program::new();
    let language = AvailableLanguages::rust();
    let path: PathBuf = ["./src"].iter().collect();

    let event_bus = Arc::new(MemoryEventBus::new());

    let config = LoadProjectConfig {
      path,
      language,
      event_bus
    };
    let mut load_project_service = LoadProjectService::new(config);
    let repo = load_project_service.run().await;

    // // Chaneg it to read
    // let parser = AvailableParsers::tree_sitter();
    // let result = match result {
    //     Ok(ok) => ok,
    //     Err(_) => SourcesOk::Nothing,
    // };
    // println!("{:?}", result);

    // let result = match result {
    //     SourcesOk::DiscoverSources(sources) => {
    //         for (uuid, file) in sources.files.iter() {
    //             parser.run(file);
    //         }
    //         Some(())
    //     }
    //     SourcesOk::Nothing => None,
    // };
}
