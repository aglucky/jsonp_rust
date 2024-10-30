use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use anyhow::Context;

#[cfg(test)]
mod tests;

pub struct JsonReader {
    reader: BufReader<File>,
    cur_line: Option<Vec<char>>,
    is_eof: bool,
    peek_char: Option<char>,
}

impl JsonReader {
    pub fn new(path: PathBuf) -> Result<Self, anyhow::Error> {
        let json_file = File::open(&path)
            .with_context(|| format!("Could not read file `{}`", path.display()))?;

        Ok(JsonReader {
            reader: BufReader::new(json_file),
            cur_line: None,
            is_eof: false,
            peek_char: None,
        })
    }
    
    pub fn peek(&mut self) -> Option<char> {
        if self.peek_char.is_none() {
            self.peek_char = self.next_internal();
        }
        self.peek_char
    }

    fn next_internal(&mut self) -> Option<char> {
        if self.is_eof {
            return None;
        }
        
        loop {
            if let Some(line) = &mut self.cur_line {
                if let Some(c) = line.pop() {
                    return Some(c);
                }
                self.cur_line = None;
                continue;
            }

            let mut next_str = String::new();
            match self.reader.read_line(&mut next_str) {
                Ok(0) => {
                    self.is_eof = true;
                    return None;
                }
                Ok(_) => {
                    self.cur_line = Some(next_str.chars().rev().collect());
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    self.is_eof = true;
                    return None;
                }
            }
        }
    }
}

impl Iterator for JsonReader {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.peek_char.take() {
            return Some(c);
        }
        self.next_internal()
    }
}
