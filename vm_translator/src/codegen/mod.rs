mod branch;
mod memory;
mod operations;

use crate::parser::command::{Command, Op};
use branch::branch_to_asm;
use memory::{pop_to_asm, push_to_asm};
use operations::op_to_asm;

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
            Command::Push { segment, index } => push_to_asm(segment, index, filename),
            Command::Pop { segment, index } => pop_to_asm(segment, index, filename),
            Command::Operation(operation) => {
                let label = self.next_label(operation);
                op_to_asm(operation, label)
            }
            Command::Branch(instruction) => branch_to_asm(&instruction),
            Command::Function(_func) => todo!(),
        };
        format!("{comment}\n{asm}")
    }
    fn next_label(&mut self, op: Op) -> u16 {
        match op {
            Op::Eq | Op::Gt | Op::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                label
            }
            _ => 0,
        }
    }
}
