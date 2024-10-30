use std::{fs::File, io::{ BufReader, Read}, path::PathBuf};
use anyhow::Context;

#[cfg(test)]
mod tests;

pub struct JsonReader {
    reader: BufReader<File>,
    is_eof: bool,
    peek_char: Option<char>,
}

impl JsonReader {
    pub fn new(path: PathBuf) -> Result<Self, anyhow::Error> {
        let json_file = File::open(&path)
            .with_context(|| format!("Could not read file `{}`", path.display()))?;

        Ok(JsonReader {
            reader: BufReader::new(json_file),
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

        let mut buf: [u8; 1] = [0; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(buf[0] as char),
            Err(_) => {
                self.is_eof = true;
                None
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
