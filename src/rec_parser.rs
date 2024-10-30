use std::collections::HashMap;
use crate::lexer::{Token, TokenReader};

const UNEXPECTED_EOF: &str = "Unexpected end of file";

#[derive(Debug, PartialEq)]
pub enum JVal {
    JString(String),
    JBool(bool),
    JNum(f64),
    JObject(std::collections::HashMap<String, JVal>),
    JArray(Vec<JVal>),
}

pub fn parse(iter: &mut TokenReader) -> JVal {
    match iter.next() {
        Some(Token::OpenObject) => parse_pairs(iter),
        Some(Token::OpenArray) => parse_array(iter),
        _ => panic!("Missing opening bracket (must be '{{' or '[')"),
    }
}

fn parse_value(iter: &mut TokenReader) -> JVal {
    let start_token = iter.peek().expect(UNEXPECTED_EOF);
    match start_token {
            Token::TString(val) => {
                iter.next();
                JVal::JString(val)
            },
            Token::TNumber(val) => {
                iter.next(); 
                JVal::JNum(val)
            },
            Token::TBool(val) => {
                iter.next();
                JVal::JBool(val)
            },
            Token::OpenObject => parse(iter),
            Token::OpenArray => parse_array(iter),
            _ => panic!("Invalid value: {:?}", start_token),
    }
}

fn parse_pairs(iter: &mut TokenReader) -> JVal {
    let mut pairs = HashMap::new();
    while let Some(token) = iter.next() {
        match token {
            Token::TString(key) => {
                if iter.next().expect(UNEXPECTED_EOF) != Token::Colon {
                    panic!("Key-value pairs must have colon")
                }
                if pairs.contains_key(&key) {
                    panic!("Only one unique key per json object allowed")
                }
                pairs.insert(key, parse_value(iter));
                continue;
            }
            Token::CloseObject => break,
            Token::Comma => continue,
            _ => panic!("Key values must be type string")

        }
    }   
    JVal::JObject(pairs)
}

fn parse_array(iter: &mut TokenReader) -> JVal {
    let mut array = Vec::new();
    while let Some(token) = iter.peek() {
        match token {
            Token::CloseArray => {
                iter.next();
                break
            },
            Token::Comma => {
                iter.next();
                continue;
            },
            _ => array.push(parse_value(iter)),
        }
    }   
    JVal::JArray(array)
}

