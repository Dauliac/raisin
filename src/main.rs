#![feature(associated_type_defaults)]
pub mod app;
pub mod core;
pub mod domain;
pub mod infra;

use app::application::Application;
use tokio::main;

#[main]
async fn main() {
    let mut app = Application::new();
    app.read_config().await;
    app.start().await;
}
