pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use std::path::PathBuf;
use app::services::Service;
use app::services::parse_project_service::{ParseProjectService, Config as ParseProjectConfig};
use app::services::load_project_service::{LoadProjectService, Config as LoadProjectConfig };
use domain::repository::Repository;
use infra::event_bus::MemoryEventBus;
use tokio::main;
use std::sync::Arc;

use domain::program::AvailableLanguages;

#[main]
async fn main() {
    // let _program = domain::program::Program::new();
    let language = AvailableLanguages::rust();
    // let path: PathBuf = ["./src"].iter().collect();
    let path: PathBuf = ["./src/"].iter().collect();

    let event_bus = Arc::new(MemoryEventBus::new());

    let config = LoadProjectConfig {
      path,
      language,
      event_bus: event_bus.clone(),
    };
    let mut load_project_service = LoadProjectService::new(config);
    let sources_repo = load_project_service.run().await;
    let sources = sources_repo.read().unwrap();
    let parse_projet_config = ParseProjectConfig {
        sources,
        event_bus: event_bus.clone(),
    };
    let mut parse_project_service = ParseProjectService::new(parse_projet_config);
    let _cfg_repo = parse_project_service.run().await;
}

pub fn fake() {
    let alo = vec!["1", "2", "!@212", "4"];
    for entry in alo {
        println!("{:#?}", &entry);
    }
}
pub struct Fake {}
impl Fake {
    pub fn fake2() {}
}
