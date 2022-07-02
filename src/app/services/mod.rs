// pub mod load_project_service;

pub trait Service<R> {
    fn run() -> R;
}
