use clap::{Parser, Subcommand};
use codecrafters_interpreter::{Expression, IlligalType, Lexer, TokenKind};
use std::{path::PathBuf, process};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
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
                if let TokenKind::Illegal(ty) = &token.kind {
                    has_error_token = true;
                    match ty {
                        IlligalType::UnterminatedString => {
                            eprintln!("[line {}] Error: Unterminated string.", token.line);
                        }
                        IlligalType::Unexpected => {
                            eprintln!(
                                "[line {}] Error: Unexpected character: {}",
                                token.line, token.literal
                            );
                        }
                    }
                } else {
                    println!("{}", token);
                }
            }

            println!("EOF  null");

            if has_error_token {
                process::exit(65);
            }
        }
        Command::Parse { filename } => {
            let file_content = std::fs::read_to_string(filename)?;
            let mut parser = codecrafters_interpreter::Parser::new(&file_content);

            let result = parser.parse().unwrap();
            match result {
                Expression::Literal(l) => {
                    println!("{}", l);
                }

                _ => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}
