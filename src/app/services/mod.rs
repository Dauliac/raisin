use async_trait::async_trait;

#[async_trait]
pub trait Service<R> {
    async fn run(&mut self) -> R;
}

// pub mod load_project_service;
// pub mod parse_project_service;
pub mod cli;
pub mod logger;
