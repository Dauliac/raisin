use std::sync::{Arc, RwLock};

use crate::domain::{program::Program, repository::Repository};

pub struct RepositoryInMemory {
    program: Option<Arc<RwLock<Program>>>,
}

impl RepositoryInMemory {
    pub fn new() -> Self {
        Self { program: None }
    }
}

impl Repository for RepositoryInMemory {
    fn read(&self) -> Option<Arc<RwLock<Program>>> {
        self.program.clone()
    }

    fn write(&mut self, aggregate: Arc<RwLock<Program>>) {
        self.program = Some(aggregate);
    }
}
