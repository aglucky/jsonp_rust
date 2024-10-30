use std::{fs::File, io::{ BufReader, Read}, path::PathBuf};
use anyhow::Context;
use std::collections::VecDeque;

#[cfg(test)]
mod tests;

const NUM_CHARS_IN_BUFFER: usize = 1024;
const BUFFER_SIZE: usize = std::mem::size_of::<char>() * NUM_CHARS_IN_BUFFER;

pub struct JsonReader {
    reader: BufReader<File>,
    is_eof: bool,
    buffer: VecDeque<char>,
}

impl JsonReader {
    pub fn new(path: PathBuf) -> Result<Self, anyhow::Error> {
        let json_file = File::open(&path)
            .with_context(|| format!("Could not read file `{}`", path.display()))?;

        Ok(JsonReader {
            reader: BufReader::new(json_file),
            is_eof: false,
            buffer: VecDeque::with_capacity(BUFFER_SIZE),
        })
    }
    
    pub fn peek(&mut self) -> Option<char> {
        if self.buffer.is_empty() {
            self.refill_buffer().ok()?;
        }
        self.buffer.front().copied()
    }

    fn refill_buffer(&mut self) -> Result<(), anyhow::Error> {
        if self.is_eof {
            return Ok(());
        }

        if !self.buffer.is_empty() {
            return Err(anyhow::anyhow!("Attempted to refill on non-empty buffer"));
        }

        let mut temp_buf = [0; BUFFER_SIZE];
            match self.reader.read(&mut temp_buf) {
                Ok(0) => {
                    self.is_eof = true;
                    return Ok(());
                }
                Ok(n) => {
                    self.buffer.extend(temp_buf[..n].iter().copied().map(char::from));
                }
                Err(e) => {
                    self.is_eof = true;
                    return Err(e).with_context(|| "Failed to read from file")?;
                }
            }

        Ok(())
    }
}

impl Iterator for JsonReader {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            self.refill_buffer().ok();
        }
        self.buffer.pop_front()
    }
}
