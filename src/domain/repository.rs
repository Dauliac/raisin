use std::sync::Arc;
use tokio::sync::RwLock;

use super::program::Program;

pub trait Repository: Send + Sync {
    // TODO check why mut
    fn read(&self) -> Option<Arc<RwLock<Program>>>;
    fn write(&mut self, aggregate: Arc<RwLock<Program>>);
}
