use super::aggregate::Cfg;

pub trait CfgsFactory<Error, Event> {
    fn register(&self) -> Result<(Vec<Cfg>, Vec<Event>), Error>;
}
