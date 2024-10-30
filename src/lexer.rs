use anyhow::Context;

use crate::reader::JsonReader;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenObject,
    CloseObject,
    Colon,
    TNumber(f64),
    TString(String),
    TBool(bool),
    TNull,
    Comma,
    OpenArray,
    CloseArray,
}

pub struct TokenReader {
    reader: JsonReader,
}

impl TokenReader {
    pub fn new(reader: JsonReader) -> Self {
        TokenReader { reader }
    }
}

impl Iterator for TokenReader {
    type Item = Result<Token, anyhow::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.reader.next() {
            let result = match ch {
                ch if ch.is_whitespace() => return self.next(),
                '{' => Ok(Token::OpenObject),
                '}' => Ok(Token::CloseObject),
                '[' => Ok(Token::OpenArray),
                ']' => Ok(Token::CloseArray),
                ':' => Ok(Token::Colon),
                ',' => Ok(Token::Comma),
                '"' => parse_string(&mut self.reader),
                '0'..='9' | '.' | '-' => parse_number(&mut self.reader, ch),
                't' | 'f' | 'T' | 'F' => parse_boolean(&mut self.reader, ch),
                'n' | 'N' => parse_null(&mut self.reader, ch),
                _ => Err(anyhow::anyhow!(
                    "Invalid character '{}' found when parsing",
                    ch
                )),
            };
            Some(result)
        } else {
            None
        }
    }
}

fn parse_string(iter: &mut JsonReader) -> Result<Token, anyhow::Error> {
    let mut string = String::new();
    while let Some(next_ch) = iter.next() {
        match next_ch {
            '"' => break,
            '\\' => {
                let escaped_ch = iter.next().ok_or_else(|| {
                    anyhow::anyhow!("Unexpected end of input after escape character")
                })?;
                match escaped_ch {
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid escape sequence '\\{}'",
                            escaped_ch
                        ))
                    }
                }
            }
            _ => string.push(next_ch),
        }
    }
    Ok(Token::TString(string))
}

fn parse_number(iter: &mut JsonReader, num_start: char) -> Result<Token, anyhow::Error> {
    let mut number = num_start.to_string();

    while let Some(next_ch) = iter.peek() {
        if next_ch.is_numeric() || next_ch == '.' {
            number.push(next_ch);
            iter.next();
        } else {
            break;
        }
    }

    if num_start == '0' && number.len() > 1 && !number.contains(".") {
        return Err(anyhow::anyhow!(
            "Only decimal numbers and 0 can start with 0"
        ));
    }

    number
        .parse::<f64>()
        .map(Token::TNumber)
        .with_context(|| format!("Invalid number: {}", number))
}

fn parse_boolean(iter: &mut JsonReader, first_char: char) -> Result<Token, anyhow::Error> {
    let expected = if first_char == 't' { "rue" } else { "alse" };

    for expected_char in expected.chars() {
        let ch = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unexpected end of input while parsing boolean"))?;
        if ch != expected_char {
            return Err(anyhow::anyhow!("Invalid boolean literal"));
        }
    }

    Ok(Token::TBool(first_char == 't'))
}

fn parse_null(iter: &mut JsonReader, first_char: char) -> Result<Token, anyhow::Error> {
    let expected = if first_char == 'n' { "ull" } else { "ULL" };

    for expected_char in expected.chars() {
        let ch = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unexpected end of input while parsing null"))?;
        if ch != expected_char {
            return Err(anyhow::anyhow!("Invalid null literal"));
        }
    }

    Ok(Token::TNull)
}
