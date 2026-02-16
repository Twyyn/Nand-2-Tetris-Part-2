use crate::{codegen::functions::translate_function, parser::command::Function};

pub fn bootstrap(filename: &str, label_count: u16) -> String {
    let call_sys_init = translate_function(
        Function::Call {
            name: "Sys.init".to_string(),
            arg_count: 0,
        },
        label_count,
    );

    format!(
        "// Bootstrap\n\
        @256\n\
        D=A\n\
        @SP
        M=D\n\
        {call_sys_init}"
    )
}
