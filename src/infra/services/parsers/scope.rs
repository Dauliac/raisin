use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use crate::app::dtos::cfg::Scope as ScopeDTO;

pub fn init() -> Result<ScopeDTO, Error> {
    let scope = ScopeDTO {
        uuid: Uuid::new_v4(),
        parent: None,
        childs: HashMap::new(),
    };
    return Ok(scope);
}

#[derive(Error, Debug)]
pub enum Error {}
