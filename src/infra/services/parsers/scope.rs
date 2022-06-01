use thiserror::Error;

use crate::app::dtos::cfg::ScopeDTO;

pub fn init() -> Result<ScopeDTO, Error> {
    let scope = ScopeDTO::new(None, None);
    return Ok(scope);
}

#[derive(Error, Debug)]
pub enum Error {}
