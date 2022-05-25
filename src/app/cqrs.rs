use std::collections::VecDeque;

use crate::core::domain::Entity;

// TODO: have a timestamp in

pub trait Command: Entity {
    fn run(&mut self);
}

pub trait Query<S, E>: Entity {
    fn run(&mut self) -> Result<S, E>;
}

pub trait Event: Entity {
    fn run(&mut self);
}

pub trait QueryHandler {
    fn handle<S, E>(&mut self, query: Box<dyn Query<S, E>>);
}

pub struct RamCommandHandler {
    queue: VecDeque<Box<dyn Command>>,
}

impl RamCommandHandler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

pub trait CommandHandler {
    fn handle(&mut self, command: Box<dyn Command>);
    fn run(&mut self);
}

impl CommandHandler for RamCommandHandler {
    fn handle(&mut self, command: Box<dyn Command>) {
        self.queue.push_back(command);
        self.run();
    }
    fn run(&mut self) {
        match self.queue.pop_front() {
            Some(mut command) => command.run(),
            None => (),
        }
    }
}

pub mod commands;
pub mod queries;
