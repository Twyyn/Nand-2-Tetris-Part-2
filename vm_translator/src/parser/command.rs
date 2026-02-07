use std::fmt::{self};

#[derive(Debug)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
}

#[derive(Debug)]
pub enum Segment {
    Constant,
    Local,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Constant => write!(f, "constant"),
            Segment::Local => write!(f, "local"),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Push { segment, index } => write!(f, "{} {}", segment, index),

            Command::Pop { segment, index } => write!(f, "{} {}", segment, index),
        }
    }
}
