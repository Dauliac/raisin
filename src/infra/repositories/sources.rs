use uuid::Uuid;

use crate::core::domain::Entity;
use crate::domain::repository::Repository;
use crate::domain::sources::aggregate::Sources;

pub struct SourcesRepository {
    uuid: Uuid,
    aggregate: Option<Box<Sources>>,
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
    fn read(&mut self, uuid: Uuid) -> Option<&Sources> {
        if self.aggregate.is_some() && uuid == self.aggregate.as_ref().unwrap().get_uuid() {
            return Some(&self.aggregate.as_ref().unwrap());
        }
        return None;
    }

    fn write(&mut self, sources: Sources) {
        self.aggregate = Some(Box::new(sources));
    }
}
