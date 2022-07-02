// TODO: have a timestamp in

pub trait Command {
    fn run(&mut self);
}

pub trait Query<R> {
    fn run(&mut self) -> R;
}
