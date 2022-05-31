use crate::core::domain::Entity;

pub trait Repository<Command, Query, QueryResult>: Entity {
    // TODO check if read need to take command
    // if we do that, we also needs to configure command
    // callback into read body
    fn read(&mut self, query: Query) -> QueryResult;
    fn write(&mut self, command: Command);
}
