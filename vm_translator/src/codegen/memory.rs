use crate::parser::command::Seg;

const PUSH_D: &str = "\
    @SP\n\
    A=M\n\
    M=D\n\
    @SP\n\
    M=M+1\n\
    ";

const POP_TO_D: &str = "\
    @SP\n\
    AM=M-1\n\
    D=M\n\
    ";

fn base_pointer(segment: Seg) -> &'static str {
    match segment {
        Seg::Local => "LCL",
        Seg::Argument => "ARG",
        Seg::This => "THIS",
        Seg::That => "THAT",
        _ => unreachable!(),
    }
}

pub fn push_to_asm(segment: Seg, index: u16, filename: &str) -> String {
    match segment {
        Seg::Constant => {
            format!(
                "\
                @{index}\n\
                D=A\n\
                {PUSH_D}\
                "
            )
        }
        Seg::Local | Seg::Argument | Seg::This | Seg::That => {
            let segment = base_pointer(segment);
            format!(
                "\
                @{segment}\n\
                D=M\n\
                @{index}\n\
                A=D+A\n\
                D=M\n\
                {PUSH_D}\
                "
            )
        }

        Seg::Static => {
            format!(
                "\
                @{filename}.{index}\n\
                D=M\n\
                {PUSH_D}\
                "
            )
        }

        Seg::Temp | Seg::Pointer => {
            let base = match segment {
                Seg::Temp => 5,
                Seg::Pointer => 3,
                _ => unreachable!(),
            };
            let addr = base + index;
            format!(
                "\
                @R{addr}\n\
                D=M\n\
                {PUSH_D}\
                "
            )
        }
    }
}

pub fn pop_to_asm(segment: Seg, index: u16, filename: &str) -> String {
    match segment {
        Seg::Constant => unreachable!(),
        Seg::Local | Seg::Argument | Seg::This | Seg::That => {
            let segment = base_pointer(segment);
            format!(
                "\
                @{segment}\n\
                D=M\n\
                @{index}\n\
                D=D+A\n\
                @R13\n\
                M=D\n\
                {POP_TO_D}\
                @R13\n\
                A=M\n\
                M=D\n\
                "
            )
        }
        Seg::Static => {
            format!(
                "\
                {POP_TO_D}\
                 @{filename}.{index}\n\
                 M=D\n\
                 "
            )
        }
        Seg::Temp | Seg::Pointer => {
            let base = match segment {
                Seg::Temp => 5,
                Seg::Pointer => 3,
                _ => unreachable!(),
            };
            let addr = base + index;
            format!(
                "\
                {POP_TO_D}\
                @R{addr}\n\
                M=D\n\
                "
            )
        }
    }
}
