use crate::core::domain::Entity;

pub trait Repository<Command, Query, Error>: Entity {
    // TODO check if read need to take command
    // if we do that, we also needs to configure command
    // callback into read body
    fn read(&mut self, query: Query) -> Result<(), Error>;
    fn write(&mut self, command: Command) -> Result<(), Error>;
}
