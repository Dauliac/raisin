use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CfgCommand {
    DiscoverCfg {
        cfg_uuid: String,
    },
    LoadBlock {
        block_uuid: String,
        precedents_uuids: Vec<String>,
        successors_uuids: Vec<String>,
    },
    LoadScope {
        scope_uuid: String,
        parent_uuid: Option<String>,
        childs_uuids: Vec<String>,
    },
}
