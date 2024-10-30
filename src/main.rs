use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;

use std::io::BufReader;
use std::process::ExitCode;

mod lexer;
mod reader;
mod parser;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let Args { path } = Args::parse();
    let json_file = File::open(&path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    let file_reader = reader::JsonReader::new(BufReader::new(json_file));
    let mut token_reader = lexer::TokenReader::new(file_reader);

    // for token in token_reader {
    //     println!{"{:?}", token}
    // }
    println!("{:?}", parser::parse(&mut token_reader));
    Ok(ExitCode::SUCCESS)
}
