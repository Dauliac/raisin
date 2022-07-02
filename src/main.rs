pub mod app;
pub mod core;
pub mod domain;
pub mod infra;
use std::path::PathBuf;
use domain::repository::Repository;
use domain::sources::factory::SourcesFactory;
use infra::event_bus::{Event, EventBus, MemoryEventBus};
use tokio::main;

use domain::program::AvailableLanguages;
use infra::factories::sources::{Config as ConfigFactory, DefaultSourcesFactory};
use infra::repositories::sources::SourcesRepository;

#[main]
async fn main() {
    // let _program = domain::program::Program::new();
    let language = AvailableLanguages::rust();
    let path: PathBuf = ["./src"].iter().collect();

    let mut event_bus = MemoryEventBus::new();

    let mut repo = SourcesRepository::new();

    let config = ConfigFactory { path, language };
    let sources_factory = DefaultSourcesFactory::new(config);
    let sources_result = sources_factory.register().await;
    if let Ok(result) = sources_result {
        let sources = result.0;
        let sources_creation_events = result.1;
        for event in sources_creation_events {
            let event = Event::Sources(event);
            event_bus.publish(event);
        }
        repo.write(sources);
    }

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
