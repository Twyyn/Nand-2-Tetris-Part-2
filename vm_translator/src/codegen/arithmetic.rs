use crate::parser::command::Operation;

fn binary_op(operation: &str) -> String {
    format!("@SP\nAM=M-1\nD=M\nA=A-1\nM=M{operation}D\n")
}

fn unary_op(operation: &str) -> String {
    format!("@SP\nA=M-1\nM={operation}M\n")
}

fn comparison_asm(prefix: &str, jump: &str, n: u16) -> String {
    let (x, y) = match jump {
        "JGT" => ("-1", "0"),
        "JLT" => ("0", "-1"),
        "JEQ" => ("0", "0"),
        _ => unreachable!(),
    };

    format!(
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         @R14\n\
         M=D\n\
         @SP\n\
         AM=M-1\n\
         D=M\n\
         @R13\n\
         M=D\n\
         @{prefix}_X_NEG_{n}\n\
         D;JLT\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XPOS_{n}\n\
         D;JLT\n\
         @{prefix}_SAFE_{n}\n\
         0;JMP\n\
         ({prefix}_X_NEG_{n})\n\
         // x < 0: check y\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XNEG_{n}\n\
         D;JGE\n\
         ({prefix}_SAFE_{n})\n\
         @R13\n\
         D=M\n\
         @R14\n\
         D=D-M\n\
         @{prefix}_TRUE_{n}\n\
         D;{jump}\n\
         @SP\n\
         A=M\n\
         M=0\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_DIFF_XPOS_{n})\n\
         @SP\n\
         A=M\n\
         M={x}\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_DIFF_XNEG_{n})\n\
         @SP\n\
         A=M\n\
         M={y}\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_TRUE_{n})\n\
         @SP\n\
         A=M\n\
         M=-1\n\
         ({prefix}_END_{n})\n\
         @SP\n\
         M=M+1\n"
    )
}

pub fn translate_arithmetic(operation: Operation, label: u16) -> String {
    match operation {
        Operation::Add => binary_op("+"),
        Operation::Sub => binary_op("-"),
        Operation::And => binary_op("&"),
        Operation::Or => binary_op("|"),
        Operation::Neg => unary_op("-"),
        Operation::Not => unary_op("!"),
        Operation::Eq => comparison_asm("EQ", "JEQ", label),
        Operation::Gt => comparison_asm("GT", "JGT", label),
        Operation::Lt => comparison_asm("LT", "JLT", label),
    }
}
