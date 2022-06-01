use uuid::Uuid;

use std::collections::HashMap;

use super::UuidDTO;

#[derive(Debug, Clone)]
pub struct CfgDTO {
    uuid: UuidDTO,
    // blocks: HashMap<Uuid, Block>,
    scopes: HashMap<UuidDTO, ScopeDTO>,
}

#[derive(Debug, Clone)]
pub struct ScopeDTO {
    uuid: UuidDTO,
    parent: Option<UuidDTO>,
    childs: HashMap<UuidDTO, ScopeDTO>,
}

impl ScopeDTO {
    pub fn new(uuid: Option<UuidDTO>, parent: Option<UuidDTO>) -> Self {
        let uuid = match uuid {
            Some(uuid) => uuid,
            None => Uuid::new_v4().to_string(),
        };

        Self {
            uuid,
            parent,
            childs: HashMap::new(),
        }
    }
}
