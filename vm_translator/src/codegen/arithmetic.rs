use indoc::formatdoc;

pub enum Arithmetic<'a> {
    Add,
    Sub,
    Neg,
    Not,
    Eq(&'a str),
    Gt(&'a str),
    Lt(&'a str),
    And,
    Or,
}

impl<'a> Arithmetic<'a> {
    pub fn to_asm(&self) -> String {
        let binary = "@SP\nAM=M-1\nD=M\nA=A-1";
        let unary = "@SP\nA=M-1";

        match self {
            Self::Add => formatdoc! {"
            {binary}
            M=M+D
        "},
            Self::Sub => formatdoc! {"
            {binary}
            M=M-D
        "},
            Self::Neg => formatdoc! {"
            {unary}
            M=-M
        "},
            Self::Not => formatdoc! {"
            {unary}
            M=-M
        "},
            Self::Eq(label) => {
                todo!()
            }
            Self::Gt(label) => {
                todo!()
            }
            Self::Lt(label) => {
                todo!()
            }
            Self::And => {
                todo!()
            }
            Self::Or => {
                todo!()
            }
        }
    }
}
