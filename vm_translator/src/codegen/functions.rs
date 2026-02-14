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
            format!(
                "\
                ({filename}.{function_name})\
                @i\n\
                M=0\n\
                (PUSH_LCL_LOOP_{label_count})\n\
                @{local_count}\n\
                D=A\n\
                @i\n\
                D=D-M\n\
                @PUSH_LCL_LOOP_END_{label_count}\n\
                D;JEQ\n\
                @LCL\n\
                D=M\n\
                @i\n\
                A=D+M\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n\
                @i\n\
                M=M+1\n\
                @PUSH_LCL_LOOP_{label_count}\n\
                0;JMP\n\
                (PUSH_LCL_LOOP_END_{label_count})\
                "
            )
        }
        FunctionCommand::Call {
            function_name,
            arg_count,
        } => todo!(),
        FunctionCommand::Return => todo!(),
    }
}
