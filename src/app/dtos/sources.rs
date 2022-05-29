use std::collections::HashMap;

pub type Uuid = String;
pub type Path = String;

#[derive(Debug, Clone)]
pub struct File {
    pub uuid: Uuid,
    pub path: Path,
    pub lines: HashMap<u64, String>,
    pub includes: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct Sources {
    pub uuid: Uuid,
    pub files: HashMap<Uuid, File>,
}

// impl SourcesCommand for Sources {
//     fn indexFile(&self, file: String) -> Result<IndexedFileEvent, Error> {}
// }

// fn apply(sources: &mut Sources, event: SourcesEvent) {
//     use SourcesEvent::*;

//     match event {
//         IndexedFile(IndexedFileEvent { path }) => {
//             bank_account.opened = true;
//             bank_account.balance = initial_balance;
//         }
//     }
// }
