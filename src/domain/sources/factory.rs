use async_trait::async_trait;

use super::aggregate::Sources;

#[async_trait]
pub trait SourcesFactory<Error, Event> {
    async fn register(&self) -> Result<(Sources, Vec<Event>), Error>;
}
