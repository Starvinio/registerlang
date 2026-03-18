use std::{env, fs, io::{self, Write}, process};

use registerlang::{Compiler, LangError, VM};

fn main() {
    let argv:Vec<String> = env::args().collect();
    println!("argv: {:?}", argv);

    // Initialize VM
    // Will persist from program start to finish
    let mut vm = VM::init();

    match {
        if argv.len() == 1 {
            run_repl(&mut vm)
        } else if argv.len() == 2 {
            run_file(&argv[1], &mut vm)
        } else {
            print_usage();
            std::process::exit(64);
        }
    } {
        Ok(_) => println!("Exited successfully"),
        Err(e) => {
            eprintln!("{e}");
            match e {
                LangError::CompileError {..} => process::exit(65),
                LangError::RuntimeError {..} => process::exit(70)
            }
        }

    }
}

fn run_repl(vm: &mut VM) -> Result<(), LangError> {
    println!("REPL Mode: Press ^D to Escape");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let line_res = match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.len() == 0 {return Ok(())}
                run(line, vm)
            }
            Err(e) => {
                eprintln!("Error: could not read line: {e}");
                continue;
            }
        };
        match line_res {
            Ok(_) => continue,
            Err(e) => eprintln!("{e}")
        }
    }
}

fn run_file(src: &str, vm: &mut VM) -> Result<(), LangError> {
    let content = match fs::read_to_string(&src) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read file '{src}': {e}");
            std::process::exit(1);
        }
    };
    run(content, vm)
}

fn run(content:String, vm:&mut VM) -> Result<(), LangError> {
    println!("{content}");
    let chunk = Compiler::init().compile(content)?;
    vm.interpret(&chunk);
    Ok(())
}

fn print_usage() {
    eprintln!("Usage:\n
        <lang-bin> <file path>\t| Run code from file\n
        <lang-bin>\t\t| Run REPL mode");
}
