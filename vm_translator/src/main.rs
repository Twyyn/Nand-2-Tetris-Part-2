use std::env;
use vm_translator::vm_translator::VMTranslator;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Usage: vm_translator <file.vm>");

    VMTranslator::new(filepath)?.run()
}
