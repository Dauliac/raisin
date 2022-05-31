use std::collections::VecDeque;

use crate::core::domain::Entity;

// TODO: have a timestamp in

pub trait Command: Entity {
    fn run(&mut self);
}

pub trait Query<R>: Entity {
    fn run(&mut self) -> R;
}

pub mod queries;
