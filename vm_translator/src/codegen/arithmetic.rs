use indoc::formatdoc;

pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Not,
    Eq(u16),
    Gt(u16),
    Lt(u16),
    And,
    Or,
}

impl Arithmetic {
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
            Self::Eq(label_num) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @EQ_TRUE_{label_num}
                D;JEQ

                @SP
                A=M
                M=0
                @EQ_END_{label_num}
                0;JMP

                (EQ_TRUE_{label_num})
                @SP
                A=M
                M=-1

                (EQ_END_{label_num})
                @SP
                M=M+1
            "},
            Self::Gt(label_num) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @GT_TRUE_{label_num}
                D;JGT

                @SP
                A=M
                M=0
                @GT_END_{label_num}
                0;JMP

                (GT_TRUE_{label_num})
                @SP
                A=M
                M=-1

                (GT_END_{label_num})
                @SP
                M=M+1
            "},
            Self::Lt(label_num) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @SP
                AM=M-1
                D=M-D

                @LT_TRUE_{label_num}
                D;JLT

                @SP
                A=M
                M=0
                @LT_END_{label_num}
                0;JMP

                (LT_TRUE_{label_num})
                @SP
                A=M
                M=-1

                (LT_END_{label_num})
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
