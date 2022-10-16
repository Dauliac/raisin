use crate::core::domain::{Aggregate, Entity};

use std::sync::Arc;

pub trait Repository<A: Aggregate<A>> {
    // TODO check why mut
    fn read(&mut self, uuid: <A as Entity<A>>::Uuid) -> Option<Arc<A>>;
    fn write(&mut self, aggregate: A);
}
