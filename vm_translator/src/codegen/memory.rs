use indoc::formatdoc;

pub enum Memory<'a> {
    Constant(u16),
    Segment(&'a str, u16),
    Static(&'a str, u16),
    Pointer(u16),
    Temp(u16),
}

impl Memory<'_> {
    pub fn push_to_asum(&self) -> String {
        let push_stack = "@SP\nA=M\nM=D\n@SP\nM=M+1";

        match self {
            Self::Constant(index) => formatdoc! {"
                @{index}
                D=A
                {push_stack}
            "},
            Self::Segment(seg, index) => formatdoc! {"
                @{seg}
                D=M
                @{index}
                A=D+A
                D=M
                {push_stack}
            "},
            Self::Static(filename, index) => formatdoc! {"
                @{filename}.{index}
                D=M
                {push_stack}
            "},
            Self::Pointer(index) => {
                let addr = 3 + index;
                formatdoc! {"
                    @R{addr}
                    D=M
                    {push_stack}
                "}
            }
            Self::Temp(index) => {
                let addr = 5 + index;
                formatdoc! {"
                    @R{addr}
                    D=M
                    {push_stack}
                "}
            }
        }
    }

    pub fn pop_to_asm(&self) -> String {
        let pop_stack = "@SP\nAM=M-1\nD=M";

        match self {
            Self::Segment(seg, index) => formatdoc! {"
                @{seg}
                D=M
                @{index}
                D=D+A
                @R13
                M=D
                {pop_stack}
                @R13
                A=M
                M=D
            "},
            Self::Static(filename, index) => formatdoc! {"
                {pop_stack}
                @{filename}.{index}
                M=D
            "},
            Self::Pointer(index) => {
                let addr = 3 + index;
                formatdoc! {"
                    {pop_stack}
                    @R{addr}
                    M=D
                "}
            }
            Self::Temp(index) => {
                let addr = 5 + index;
                formatdoc! {"
                    {pop_stack}
                    @R{addr}
                    M=D
                "}
            }

            _ => unreachable!(),
        }
    }
}
