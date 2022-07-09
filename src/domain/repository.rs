use crate::core::domain::{Aggregate, Uuid};

use std::sync::Arc;

pub trait Repository<A: Aggregate> {
    // TODO check if read need to take command
    // if we do that, we also needs to configure command
    // callback into read body
    fn read(&mut self, uuid: Uuid) -> Option<Arc<A>>;
    fn write(&mut self, aggregate: A);
}
