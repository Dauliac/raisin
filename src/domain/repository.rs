use crate::core::domain::{Aggregate, Uuid};

use std::sync::Arc;

pub trait Repository<A: Aggregate> {
    // TODO check why mut
    fn read(&mut self, uuid: Uuid) -> Option<Arc<A>>;
    fn write(&mut self, aggregate: A);
}
