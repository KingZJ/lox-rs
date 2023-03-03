use std::env::args;
use std::io::{Read, BufReader, self, BufRead};
use std::fs::File;
use std::str;

use lox_ast::scanner::Scanner;
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]).expect("cannot run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let f = File::open(path).expect("cannot open file");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;
    run(&buf);

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        println!(">");

        for line in stdin.lock().lines() {
            let line_content = line.unwrap();
            if line_content.is_empty() {
                break;
            }
            run(line_content.as_bytes());
        }
    }
}

fn run(source: &[u8]) {
    let mut scanner = Scanner::new(String::from(str::from_utf8(source).unwrap()));
    for i_byte in scanner.scan_tokens().unwrap() {
        println!("{}", i_byte);
    }
}

