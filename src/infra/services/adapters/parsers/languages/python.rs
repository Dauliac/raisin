use std::sync::Arc;

use rustpython_parser::{ast, parser};
use tokio::sync::RwLock;

use crate::{app::cqrs_es::cqrs::CommandBus, domain::{program::Program, repository::Repository}};

pub struct Config {
    pub repository: Arc<RwLock<dyn Repository>>,
    pub command_bus: Arc<RwLock<dyn CommandBus>>,
}

pub struct PythonParser {
    config: Config,
}

impl PythonParser {
    pub async fn run(&self) {
        let repo = self.config.repository
          .read()
          .await;
        let program = repo
          .read().expect("No program in repository");
        let program_locked = program.read().await;

        let sources_path = program_locked
          .path();
        let sources_path = sources_path
           .to_str()
          .expect("Program sources path is empty");
        let python_ast = parser::parse_program(sources_path);
        let python_ast = match python_ast {
            Ok(parsed) => parsed,
            Err(error) => {
                !todo!("Error")
            }
        };
        for statement in python_ast.statements.iter() {
            match statement.node {
                ast::StatementType::FunctionDef {
                    is_async,
                    name,
                    args,
                    body,
                    decorator_list,
                    returns
                } => {
                    // todo new cfg here
                    for statement  in body.iter() {
                    }
                }
                _ => {}
            }

        }
    }
}
