use crate::{Chunk, Instruction, LangError, OpCode, Parser, VM, Value};
use std::{mem, process};

/// Used to call methods in main() without having to deal with errors seperately
pub fn quickres(r: Result<(), LangError>) {
    match r {
        Ok(_) => return,
        Err(e) => {
            println!("{:#?}", e);
            process::exit(e.exit_code());
        }
    }
}

/// Used to test the frontend via string input
pub fn test_parser(src: &str) -> Result<(), LangError> {
    println!("\nTESTING PARSER FOR: {:?}", src);
    let parser = Parser::init(src.to_string().into_boxed_str())?;
    let chunk = parser.compile()?;
    print_instr(&chunk);
    Ok(())
}

pub fn test_expr(src: &str, vm: &mut VM) -> Result<(), LangError> {
    println!("\nTESTING EXPRESSION: {:?}", src);
    let boxed_str = src.to_string().into_boxed_str();
    vm.interpret(boxed_str)?;
    Ok(())
}

pub fn print_instr(chunk: &Chunk) {
    println!("Instructions\n=======");
    for instr in &chunk.instructions {
        println!("{instr}");
    }
}

pub fn struct_sizes() {
    println!("Structure Sizes\n=================");
    println!("Size of OpCode: {}", mem::size_of::<OpCode>());
    println!("Size of Instruction: {}", mem::size_of::<Instruction>());
    println!("Size of Value: {}", mem::size_of::<Value>());
}
