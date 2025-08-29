use clap::{Parser, Subcommand};
use codecrafters_interpreter::{Lexer, TokenKind};
use std::{path::PathBuf, process};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
}

fn main() -> codecrafters_interpreter::Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Tokenize { filename } => {
            let file_content = std::fs::read_to_string(filename)?;
            let lexer = Lexer::new(&file_content);
            let mut has_error_token = false;

            let tokens: Vec<_> = lexer.collect();

            for token in &tokens {
                if token.kind == TokenKind::Illegal {
                    has_error_token = true;
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        token.line, token.literal
                    );
                } else {
                    println!("{}", token);
                }
            }

            println!("EOF  null");

            if has_error_token {
                process::exit(65);
            }
        }
    }
    Ok(())
}
