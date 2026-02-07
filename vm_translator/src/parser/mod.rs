pub mod command;
use command::{Command, Segment};

pub struct Parser {
    lines: Vec<String>,
    current: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let lines: Vec<String> = source.lines().map(String::from).collect();
        Self { lines, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        todo!()
    }

    fn next_command() {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.lines.len()
    }
}
