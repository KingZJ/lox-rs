use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::str;

use lox_ast::error::LoxError;
use lox_ast::interpreter::Interpreter;
use lox_ast::parser::Parser;
use lox_ast::scanner::Scanner;
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("cannot run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let mut f = File::open(path).expect("cannot open file");
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    match run(buf) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(64);
        }
    }

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        for line in stdin.lock().lines() {
            let line_content = line.unwrap();
            if line_content.is_empty() {
                break;
            }
            match run(line_content) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{:?}", e);
                    // std::process::exit(64);
                }
            }

            print!("> ");
            io::stdout().flush().unwrap();
        }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for i_byte in tokens {
        println!("{:?}", i_byte);
    }

    let mut parser = Parser::new(tokens);
    let interpreter = Interpreter {};
    interpreter.interpreter(&parser.parse()?);

    Ok(())
}
