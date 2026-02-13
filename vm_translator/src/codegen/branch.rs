use crate::parser::command::Br;

pub fn branch_to_asm(instruction: &Br) -> String {
    match instruction {
        Br::Label { label } => {
            format!("({label})\n")
        }
        Br::Goto { label } => {
            format!(
                "\
                @{label}\n\
                A=M\n\
                "
            )
        }
        Br::IfGoto { label } => {
            format!(
                "\
                @SP\n\
                AM=M-1\n\
                D=M\n\
                @{label}\n\
                D;JNE\n\
                "
            )
        }
    }
}
