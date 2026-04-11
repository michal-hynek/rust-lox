use std::{fs::read_to_string, io::{self, Write}};

use anyhow::Result;

use crate::{interpreter::{self, Interpreter}, parser::{Parser, ast_printer::AstPrinter}, scanner::Scanner};

pub fn run_file(file_path: &str) -> Result<()> {
    let source = read_to_string(file_path)?;
    run(&source)
}

pub fn run_prompt() -> Result<()> {
    println!("Lox REPL (press CTRL-D to exit)");

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    println!();
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            },
        }

        let _ = run(&input).inspect_err(|e| eprintln!("{e}"));

        input.clear();
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    let interpreter = Interpreter {};
    interpreter.interpret(&expr)?;

    Ok(())
}