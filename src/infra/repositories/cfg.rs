// use std::collections::HashMap;
// use std::sync::Arc;

// use crate::core::domain::{Entity, Uuid};
// use crate::domain::cfg::aggregate::Cfg;
// use crate::domain::program::Program;
// use crate::domain::repository::Repository;

// pub struct RepositoryInMemory {
//     uuid: Uuid,
//     program: Program,
// }

// impl Entity<CfgRepository> for CfgRepository {
//     type Uuid = Uuid;
//     fn get_uuid(&self) -> Self::Uuid {
//         self.uuid
//     }

//     fn equals(&self, entity: Entity<CfgRepository>) -> bool {
//         self.uuid == entity.get_uuid()
//     }
// }

// impl CfgRepository {
//     pub fn new() -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             aggregates: HashMap::new(),
//         }
//     }
// }

// impl Repository<Cfg> for CfgRepository {
//     fn read(&mut self, uuid: Uuid) -> Option<Arc<Cfg>> {
//         match self.aggregates.get(&uuid).as_deref() {
//             Some(cfg) => Some(cfg.clone()),
//             None => None,
//         }
//     }

//     fn write(&mut self, cfg: Cfg) {
//         self.aggregates.insert(cfg.get_uuid(), Arc::new(cfg));
//     }
// }
