mod ast;
mod parser;
mod print;
mod scanner;

use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::num;
use std::path::Path;
use std::result;

use clap::Parser;

use self::scanner::Scanner;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseNumber(#[from] num::ParseFloatError),
    #[error("Error interpreting line \"{line_string}\": {message}")]
    InterpretError {
        line_string: String,
        message: String,
    },
    #[error("Error interpreting line [{line_index}] \"{line_string}\": {message}")]
    InterpretErrorWithIndex {
        line_index: usize,
        line_string: String,
        message: String,
    },
}

impl Error {
    pub fn message(line_str: &str, message: &str) -> Self {
        Error::InterpretError {
            line_string: line_str.to_string(),
            message: message.to_string(),
        }
    }

    pub fn message_with_line_index(line_index: usize, line_str: &str, message: &str) -> Self {
        Error::InterpretErrorWithIndex {
            line_index,
            line_string: line_str.to_string(),
            message: message.to_string(),
        }
    }
}

type Result<T> = result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(file) = &args.file {
        run_file(file)?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut source = String::new();

    file.read_to_string(&mut source)?;
    interpret(source)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    print!("> ");
    io::stdout().flush()?;

    for line in io::stdin().lines().flatten() {
        interpret(line)?;

        print!("> ");
        io::stdout().flush()?;
    }

    Ok(())
}

fn interpret(source: String) -> Result<()> {
    let tokens = Scanner::new(source).read_tokens()?;
    println!("Found tokens: {tokens:?}");

    Ok(())
}
