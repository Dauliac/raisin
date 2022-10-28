use crate::{
    app::cqrs_es::event::{EventHandler, Events},
    infra::services::logger::SimpleLogger,
};

pub trait Logger: EventHandler + Send {
    fn log(&self, event: Events);
}

pub enum Loggers {
    Default(SimpleLogger),
}
