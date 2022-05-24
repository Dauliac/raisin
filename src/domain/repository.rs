use crate::core::domain::Entity;

pub trait Repository<Aggregate, Error>: Entity {
    // TODO check if read need to take command
    // if we do that, we also needs to configure command
    // callback into read body
    fn read(&mut self) -> Result<Aggregate, Error>;
}
