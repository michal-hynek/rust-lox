use std::{fs::read_to_string, io::{self, Write}};

use anyhow::Result;

use crate::scanner::Scanner;

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

        run(&input)?;
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}