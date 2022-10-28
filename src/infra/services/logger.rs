extern crate pretty_env_logger;

use log::{info, warn};

use crate::app::cqrs_es::event::{EventHandler, Events};
use crate::app::services::logger::Logger;

pub struct SimpleLogger {}

impl SimpleLogger {
    pub fn new() -> Self {
        pretty_env_logger::init();
        Self {}
    }
}

impl EventHandler for SimpleLogger {
    fn notify(&self, event: Events) {
        self.log(event)
    }
}

impl Logger for SimpleLogger {
    fn log(&self, event: Events) {
        match event {
            Events::Domain(event) => {
                let alo = serde_json::to_string(&event);
                match serde_json::to_string(&event) {
                    Ok(json) => {
                        info!("{}", json.to_owned());
                    }
                    Err(_) => {
                        panic!("Invalid events format in logger matching domain event");
                    }
                }
            }
            Events::DomainError(event) => match serde_json::to_string(&event) {
                Ok(json) => {
                    warn!("{}", json);
                }
                Err(_) => {
                    panic!("Invalid events format in logger matching domain error");
                }
            },
        }
    }
}
