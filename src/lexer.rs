use crate::reader::JsonReader;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenObject,
    CloseObject,
    Colon,
    TNumber(f64),
    TString(String),
    TBool(bool),
    Comma,
    OpenArray,
    CloseArray,
}

pub(crate) struct TokenReader {
    reader: JsonReader,
}

impl TokenReader {
    pub fn new(reader: JsonReader) -> Self {
        TokenReader {
            reader,
        }
    }
}

impl Iterator for TokenReader {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.reader.next() {
            match ch {
                ch if ch.is_whitespace() => return self.next(),
                '{' => return Some(Token::OpenObject),
                '}' => return Some(Token::CloseObject),
                '[' => return Some(Token::OpenArray),
                ']' => return Some(Token::CloseArray),
                ':' => return Some(Token::Colon),
                ',' => return Some(Token::Comma),
                '"' => return Some(parse_string(&mut self.reader)),
                '0'..='9' | '.' => return Some(parse_number(&mut self.reader, ch)),
                't' | 'f' | 'T' | 'F' => return Some(parse_boolean(&mut self.reader, ch)),
                _ => panic!("Invalid character found when parsing")
            }
        }
        None
    }
}


fn parse_string(iter: &mut JsonReader) -> Token {
    let mut string = String::new();
    while let Some(next_ch) = iter.next() {
        if next_ch == '"' {
            break;
        }
        string.push(next_ch);
    }

    return Token::TString(string);
}

fn parse_number(iter: &mut JsonReader, num_start: char) -> Token {  
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
        panic!("Only decimal numbers and 0 can start with 0");
    }

    return Token::TNumber(number.parse::<f64>().expect("Invalid number"));
}

fn parse_boolean(iter: &mut JsonReader, first_char: char) -> Token {
    let expected = if first_char == 't' {
        "rue"
    } else {
        "alse"
    };

    for expected_char in expected.chars() {
        if let Some(ch) = iter.next() {
            if ch != expected_char {
                panic!("Invalid boolean literal");
            }
        } else {
            panic!("Unexpected end of input while parsing boolean");
        }
    }

    Token::TBool(first_char == 't')
}