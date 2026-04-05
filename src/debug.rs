use crate::{ LangError, Parser, Instruction, Chunk };
use std::process;

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
    println!("\nTESTING PARSER FOR: \"{src}\"");
    let parser = Parser::init(src.to_string().into_boxed_str())?;
    let chunk = parser.compile()?;
    print_instr(&chunk);
    Ok(())
}

pub fn print_instr(chunk: &Chunk) {
    println!("Instructions\n=======");
    for instr in &chunk.instructions {
        println!("{instr}");
    }
}
