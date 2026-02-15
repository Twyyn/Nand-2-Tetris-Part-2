use crate::parser::command::FunctionCommand;

pub fn compile_function(
    function_command: FunctionCommand,
    filename: &str,
    label_count: u16,
) -> String {
    match function_command {
        FunctionCommand::Function {
            function_name,
            local_count,
        } => {
            let mut asm = format!("({filename}.{function_name})\n");

            if local_count <= 8 {
                for _ in 0..local_count {
                    asm.push_str(
                        "@SP\n\
                                A=M\n\
                                M=0\n\
                                @SP\n\
                                M=M+1\n",
                    );
                }
            } else {
                asm.push_str(&format!(
                    "@{local_count}\n\
                     D=A\n\
                     @R13\n\
                     M=D\n\
                     (INIT_LOCALS_{label_count})\n\
                     @R13\n\
                     D=M\n\
                     @END_INIT_{label_count}\n\
                     D;JEQ\n\
                     @SP\n\
                     A=M\n\
                     M=0\n\
                     @SP\n\
                     M=M+1\n\
                     @R13\n\
                     M=M-1\n\
                     @INIT_LOCALS_{label_count}\n\
                     0;JMP\n\
                     (END_INIT_{label_count})\n"
                ));
            }
            asm
        }

        FunctionCommand::Call {
            function_name,
            arg_count,
        } => {
            format!(
                "// call {function_name} {arg_count}\n\
                 @{filename}.{function_name}$ret.{label_count}\n\
                 D=A\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 // push LCL\n\
                 @LCL\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 // push ARG\n\
                 @ARG\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 // push THIS\n\
                 @THIS\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 // push THAT\n\
                 @THAT\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 // ARG = SP - 5 - arg_count\n\
                 @SP\n\
                 D=M\n\
                 @5\n\
                 D=D-A\n\
                 @{arg_count}\n\
                 D=D-A\n\
                 @ARG\n\
                 M=D\n\
                 // LCL = SP\n\
                 @SP\n\
                 D=M\n\
                 @LCL\n\
                 M=D\n\
                 @{filename}.{function_name}\n\
                 0;JMP\n\
                 ({filename}.{function_name}$ret.{label_count})\n\
                 "
            )
        }

        FunctionCommand::Return => {
            format!(
                "// return\n\
                @LCL\n\
                D=M\n\
                @R13\n\
                M=D\n\
                @5\n\
                A=D-A\n\
                D=M\n\
                @R14\n\
                M=D\n\
                @SP\n\
                AM=M-1\n\
                D=M\n\
                @ARG\n\
                A=M\n\
                M=D\n\
                @ARG\n\
                D=M+1\n\
                @SP\n\
                M=D\n\
                @R13\n\
                AM=M-1\n\
                D=M\n\
                @THAT\n\
                M=D\n\
                @R13\n\
                AM=M-1\n\
                D=M\n\
                @THIS\n\
                M=D\n\
                 // ARG = *(frame - 3)\n\
                 @R13\n\
                 AM=M-1\n\
                 D=M\n\
                 @ARG\n\
                 M=D\n\
                 @R13\n\
                 AM=M-1\n\
                 D=M\n\
                 @LCL\n\
                 M=D\n\
                 @R14\n\
                 A=M\n\
                 0;JMP\n\
                 "
            )
        }
    }
}
