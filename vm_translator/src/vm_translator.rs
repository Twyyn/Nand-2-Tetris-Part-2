use crate::codegen::CodeGen;
use crate::parser::Parser;
use crate::parser::command::Command;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[allow(unused)]
pub struct VMTranslator {
    filename: String,
    commands: Vec<Command>,
    output_path: PathBuf,
}

impl VMTranslator {
    pub fn new(filepath: &str) -> Result<Self, String> {
        let filename = Path::new(filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?
            .to_string();

        let source =
            fs::read_to_string(filepath).map_err(|e| format!("Failed to read {filepath}: {e}"))?;

        let commands = Parser::parse(&source)
            .map_err(|(line, e)| format!("Parse error at line {line}: {e}"))?;

        let output_path = Path::new(filepath).with_extension("asm");

        Ok(Self {
            filename,
            commands,
            output_path,
        })
    }

    pub fn run(self) -> Result<(), String> {
        let mut codegen = CodeGen::new();

        let file = fs::File::create(&self.output_path)
            .map_err(|e| format!("Could not create file: {e}"))?;
        let mut writer = BufWriter::new(file);

        for cmd in self.commands {
            let asm = codegen.translate(cmd, &self.filename);
            writeln!(writer, "{}", asm).map_err(|e| e.to_string())?;
        }
        writer.flush().map_err(|e| e.to_string())?;

        Ok(())
    }
}
