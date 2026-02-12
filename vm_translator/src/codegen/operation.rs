use crate::parser::command::OP;

fn binary_op(op: &str) -> String {
    format!(
        "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        A=A-1\n\
        M=M{op}D\n\
        "
    )
}

fn unary_op(op: &str) -> String {
    format!(
        "\
        @SP\n\
        A=M-1\n\
        M={op}M\n\
        "
    )
}

fn comparison_asm(prefix: &str, jump: &str, n: u16) -> String {
    format!(
        "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @{prefix}_TRUE_{n}\n\
        D;{jump}\n\
        @SP\n \
        A=M\n \
        M=0\n\
        @{prefix}_END_{n}\n\
        0;JMP\n\
        ({prefix}_TRUE_{n})\n\
        @SP\n\
        A=M\n\
        M=-1\n\
        \t({prefix}_END_{n})\n\
        @SP\n\
        M=M+1\n\
        "
    )
}

pub fn op_to_asm(op: OP, label: u16) -> String {
    match op {
        OP::Add => binary_op("+"),
        OP::Sub => binary_op("-"),
        OP::And => binary_op("&"),
        OP::Or => binary_op("|"),
        OP::Neg => unary_op("-"),
        OP::Not => unary_op("!"),
        OP::Eq => comparison_asm("EQ", "JEQ", label),
        OP::Gt => comparison_asm("GT", "JGT", label),
        OP::Lt => comparison_asm("LT", "JLT", label),
    }
}
