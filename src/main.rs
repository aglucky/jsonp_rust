use anyhow::Result;
use clap::Parser;
use parser::JVal;
use std::process::ExitCode;

pub mod lexer;
pub mod parser;
pub mod reader;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let Args { path } = Args::parse();

    let file_reader = reader::JsonReader::new(path)?;
    let mut token_reader = lexer::TokenReader::new(file_reader);

    match parser::parse(&mut token_reader) {
        Ok(JVal::JArray(array)) => println!("{:#?}", array),
        Ok(JVal::JObject(obj)) => println!("{:#?}", obj),
        Ok(_) => return Err(anyhow::anyhow!("JSON document must be an array or object").into()),
        Err(e) => return Err(e.into()),
    };

    Ok(ExitCode::SUCCESS)
}
