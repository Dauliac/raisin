pub type Uuid = String;

pub mod cfg;
pub mod sources;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cfg {
    uuid: Uuid,
    // blocks: HashMap<Uuid, Block>,
    scopes: HashMap<Uuid, Scope>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    uuid: Uuid,
    parent: Option<Uuid>,
    childs: HashMap<Uuid, Scope>,
}
