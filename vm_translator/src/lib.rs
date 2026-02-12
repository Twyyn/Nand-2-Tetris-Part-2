mod codegen;
mod error;
mod parser;

use codegen::CodeGen;
use error::VMError;
use parser::{Parser, command::Command};
use std::{
    ffi::OsStr,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct VMTranslator {
    filename: String,
    commands: Vec<Command>,
    output_path: PathBuf,
}

#[allow(clippy::missing_errors_doc)]
impl VMTranslator {
    pub fn new(filepath: &str) -> Result<Self, VMError> {
        let filepath = Path::new(filepath);

        if filepath.extension() != Some(OsStr::new("vm")) {
            return Err(VMError::InvalidInput(
                "File must have .vm extension".to_string(),
            ));
        }

        let filename = filepath
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(VMError::InvalidInput("Invalid filename".to_string()))?
            .to_string();

        let source = fs::read_to_string(filepath)?;
        let commands = Parser::parse(&source)?;
        let output_path = filepath.with_extension("asm");

        Ok(Self {
            filename,
            commands,
            output_path,
        })
    }

    pub fn run(self) -> Result<(), VMError> {
        let mut codegen = CodeGen::new();

        let file = fs::File::create(&self.output_path)?;
        let mut writer = BufWriter::new(file);

        for command in self.commands {
            let asm = codegen.translate(&command, &self.filename);
            writeln!(writer, "{asm}")?;
        }
        writer.flush()?;

        Ok(())
    }
}
