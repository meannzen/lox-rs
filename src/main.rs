use clap::{Parser, Subcommand};
use codecrafters_interpreter::Lexer;
use std::path::PathBuf;

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
            let mut lexer = Lexer::new(&file_content);
            let token = lexer.next().unwrap();
            println!("{token}");
        }
    }
    Ok(())
}
