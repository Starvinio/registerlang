use std::{
    env, fs,
    io::{self, Write},
    process,
};

use registerlang::{LangError, VM, debug};

/// Main entry point collects arguments
/// and runs program accordingly
fn main() {
    let mut vm = VM::init();

    debug::quickres(debug::test_expr("2 + 2", &mut vm));
    debug::quickres(debug::test_expr("2 + 2 * 2", &mut vm));
    debug::quickres(debug::test_expr("2 * 2 + 2", &mut vm));
    debug::quickres(debug::test_expr("2 + 2 == 4", &mut vm));
    debug::quickres(debug::test_expr("2 + 2 >= 4", &mut vm));
    debug::quickres(debug::test_expr("--2 + --2 >= 4", &mut vm));
    debug::quickres(debug::test_expr("(2 + 2) * 4", &mut vm));
    debug::quickres(debug::test_expr("(2 + 3 == 5) == true", &mut vm));
    debug::quickres(debug::test_expr("false == !true", &mut vm));
    debug::quickres(debug::test_expr("999^0", &mut vm));
    debug::quickres(debug::test_expr("0^999", &mut vm));
    debug::quickres(debug::test_expr("1 + 2^10", &mut vm));

    process::exit(420); // debug exit

    let argv: Vec<String> = env::args().collect();

    // Initialize VM
    // Will persist from program start to finish
    let mut vm = VM::init();
    match argv.len() {
        1 => run_repl(&mut vm),
        2 => run_file(&argv[1], &mut vm),
        _ => {
            print_usage();
            process::exit(64);
        }
    }
    process::exit(0);
}

fn run_repl(vm: &mut VM) {
    println!("REPL Mode: Press ^D to Escape");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let line_res = match io::stdin().read_line(&mut line) {
            Ok(_) => {
                // Check for ^D Press
                if line.len() == 0 {
                    println!();
                    return;
                }

                run(line.clone(), vm)
            }
            Err(e) => {
                eprintln!("Error: could not read line: {e}");
                process::exit(74);
            }
        };
        match line_res {
            Ok(_) => continue,
            Err(e) => e.print_error(&line),
        }
    }
}

fn run_file(src: &str, vm: &mut VM) {
    let content = match fs::read_to_string(&src) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read file '{src}': {e}");
            process::exit(66);
        }
    };
    match run(content, vm) {
        Ok(_) => return,
        Err(e) => {
            e.print_error(src);
            process::exit(e.exit_code());
        }
    }
}

fn run(content: String, vm: &mut VM) -> Result<(), LangError> {
    vm.interpret(content.into_boxed_str())
}

fn print_usage() {
    eprintln!(
        "Usage:\n
        <lang-bin> <file path>\t| Run code from file\n
        <lang-bin>\t\t| Run REPL mode"
    );
}
