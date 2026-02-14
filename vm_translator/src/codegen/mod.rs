mod branching;
mod functions;
mod memory;
mod operations;

use crate::parser::command::{Command, OperationCommand};
use branching::compile_branch;
use memory::{compile_pop, compile_push};
use operations::compile_operation;

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
}

impl CodeGen {
    pub fn new() -> Self {
        Self { label_count: 0 }
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        let comment = format!("// {command}");
        let asm = match command {
            Command::Push { segment, index } => compile_push(segment, index, filename),
            Command::Pop { segment, index } => compile_pop(segment, index, filename),
            Command::Operation(operation_command) => {
                let label = self.next_label(operation_command);
                compile_operation(operation_command, label)
            }
            Command::Branching(branch_command) => compile_branch(branch_command),
            Command::Function(_function_command) => todo!(),
        };
        format!("{comment}\n{asm}")
    }
    fn next_label(&mut self, operation_command: OperationCommand) -> u16 {
        match operation_command {
            OperationCommand::Eq | OperationCommand::Gt | OperationCommand::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                label
            }
            _ => 0,
        }
    }
}
