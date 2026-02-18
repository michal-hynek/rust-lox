use std::{fs::read_to_string, io::{self, Write}};

use anyhow::Result;

pub fn run_file(file_path: &str) -> Result<()> {
    let source = read_to_string(file_path)?;
    run(&source)
}

pub fn run_prompt() -> Result<()> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
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
    todo!("implement run");
}