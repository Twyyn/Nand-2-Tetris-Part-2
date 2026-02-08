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
        match self {
            Self::Add => formatdoc! {"
                @SP
                AM=M-1
                D=M
                A=A-1
                M=M+D
            "},
            Self::Sub => formatdoc! {"
                @SP
                AM=M-1
                D=M
                A=A-1
                M=M-D
            "},
            Self::Neg => formatdoc! {"
                @SP
                A=M-1
                M=-M
            "},
            Self::Not => formatdoc! {"
                @SP
                A=M-1
                M=!M
            "},
            Self::Eq(label) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @EQ_TRUE_{label}
                D;JEQ

                @SP
                A=M
                M=0
                @EQ_END_{label}
                0;JMP

                (EQ_TRUE_{label})
                @SP
                A=M
                M=-1

                (EQ_END_{label})
                @SP
                M=M+1
            "},
            Self::Gt(label) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @GT_TRUE_{label}
                D;JGT

                @SP
                A=M
                M=0
                @GT_END_{label}
                0;JMP

                (GT_TRUE_{label})
                @SP
                A=M
                M=-1

                (GT_END_{label})
                @SP
                M=M+1
            "},
            Self::Lt(label) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @LT_TRUE_{label}
                D;JLT

                @SP
                A=M
                M=0
                @LT_END_{label}
                0;JMP

                (LT_TRUE_{label})
                @SP
                A=M
                M=-1

                (LT_END_{label})
                @SP
                M=M+1
            "},
            Self::And => formatdoc! {"
                @SP
                AM=M-1
                D=M
                A=A-1
                M=M&D
            "},
            Self::Or => formatdoc! {"
                @SP
                AM=M-1
                D=M
                A=A-1
                M=M|D
            "},
        }
    }
}
