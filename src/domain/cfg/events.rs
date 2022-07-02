use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CfgEvent {
    CfgDiscovered {
        cfg_uuid: String,
    },
    CfgParsed {
        cfg_uuid: String,
    },
    BlockLoaded {
        block_uuid: String,
        precedents_uuids: Vec<String>,
        successors_uuids: Vec<String>,
    },
    ScopeLoaded {
        scope_uuid: String,
        parent_uuid: Option<String>,
        childs_uuids: Vec<String>,
    },
}

impl for CfgEvent {
    fn event_type(&self) -> String {
        match self {
            CfgEvent::CfgDiscovered { .. } => "CfgDiscovered".to_string(),
            CfgEvent::CfgParsed { .. } => "CfgParsed".to_string(),
            CfgEvent::BlockLoaded { .. } => "BlockLoaded".to_string(),
            CfgEvent::ScopeLoaded { .. } => "ScopeLoaded".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("given sources was not loaded")]
    CfgNotParsable,
}
