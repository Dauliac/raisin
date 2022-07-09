use std::sync::Arc;

use uuid::Uuid;

use crate::core::domain::Entity;
use crate::domain::repository::Repository;
use crate::domain::sources::aggregate::Sources;

pub struct SourcesRepository {
    uuid: Uuid,
    aggregate: Option<Arc<Sources>>,
}

impl Entity for SourcesRepository {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn equals(&self, entity: Box<dyn Entity>) -> bool {
        self.uuid == entity.get_uuid()
    }
}

impl SourcesRepository {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            aggregate: None,
        }
    }
}

impl Repository<Sources> for SourcesRepository {
    fn read(&mut self, uuid: Uuid) -> Option<Arc<Sources>> {
        let sources = self.aggregate.clone();
        if sources.is_some() && uuid == sources.clone().unwrap().get_uuid() {
            return Some(sources.unwrap());
        }
        return None;
    }

    fn write(&mut self, sources: Sources) {
        self.aggregate = Some(Arc::new(sources));
    }
}
