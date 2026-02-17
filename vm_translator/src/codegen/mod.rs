mod arithmetic;
mod branching;
mod functions;
mod memory;

use crate::parser::command::Function;
use crate::{codegen::functions::translate_function, parser::command::Command};
use arithmetic::translate_arithmetic;
use branching::translate_branch;
use memory::{translate_pop, translate_push};

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
    current_function: Option<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        match command {
            Command::Push { segment, index } => translate_push(segment, index, filename),
            Command::Pop { segment, index } => translate_pop(segment, index, filename),
            Command::Arithmetic(operation) => {
                let label = self.next_label();
                translate_arithmetic(operation, label)
            }
            Command::Branching(branch) => {
                translate_branch(branch, self.current_function.as_deref().unwrap_or(""))
            }
            Command::Function(function) => {
                if let Function::Declare { ref name, .. } = function {
                    self.current_function = Some(name.clone());
                }
                let label = self.next_label();
                translate_function(function, label)
            }
        }
    }

    pub fn emit_bootstrap(&mut self) -> String {
        let label = self.next_label();
        let call_sys_init = translate_function(
            Function::Call {
                name: "Sys.init".to_string(),
                arg_count: 0,
            },
            label,
        );
        format!(
            "// Bootstrap\n\
             @256\n\
             D=A\n\
             @SP\n\
             M=D\n\
             {call_sys_init}"
        )
    }

    fn next_label(&mut self) -> u16 {
        let n = self.label_count;
        self.label_count += 1;
        n
    }
}
