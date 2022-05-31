use super::super::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    uuid: Uuid,
    parent: Option<Uuid>,
    childs: HashMap<Uuid, Scope>,
}
