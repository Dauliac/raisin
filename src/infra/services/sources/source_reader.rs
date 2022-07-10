use std::fs::File;
use std::io::{BufReader, BufRead};
use thiserror::Error;

use crate::core::domain::{Aggregate, Entity};
use crate::domain::sources::aggregate::{Sources, SourcesEvent};

pub struct Config<'a> {
    pub sources: &'a mut Sources,
}

pub struct SourceReader<'a> {
    config: Config<'a>,
}

impl SourceReader<'_> {
    pub fn new(config: Config) -> SourceReader {
        SourceReader { config }
    }

    pub async fn run(&mut self) -> Result<Vec<<Sources as Aggregate>::Event>, Error> {
        let mut events = Vec::new();
        for (_, file) in self.config.sources.edit_files() {
            let reader = File::open(file.get_path()).unwrap();
            let reader = BufReader::new(reader);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); // Ignore errors.
                let index: usize = index.try_into().unwrap();
                file.insert_line(index, line);
                events.push(
                  SourcesEvent::FileContentLoaded {
                    file_uuid: file.get_uuid(),
                  }
                )
            }
        }
        Ok(events)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to register sources: ${0}")]
    DomainError(<Sources as Aggregate>::Error),
}
