use crate::lexer::{Token, TokenReader};
use std::collections::HashMap;

const UNEXPECTED_EOF: &str = "Unexpected end of file";

#[derive(Debug, PartialEq, Clone)]
pub enum JVal {
    JString(String),
    JBool(bool),
    JNum(f64),
    JObject(std::collections::HashMap<String, JVal>),
    JArray(Vec<JVal>),
}

#[derive(Debug)]
enum ParseState {
    Object(HashMap<String, JVal>),
    Array(Vec<JVal>),
    Value(JVal)
}

pub fn parse(iter: &mut TokenReader) -> JVal {
    let mut stack: Vec<ParseState> = Vec::new();
    let mut cur_key:Option<String> = None;

    match iter.next() {
        Some(Token::OpenObject) => stack.push(ParseState::Object(HashMap::new())),
        Some(Token::OpenArray) => stack.push(ParseState::Array(Vec::new())),
        _ => panic!("Missing opening bracket (must be '{{' or '[')"),
    }

    while !stack.is_empty() {
        let last_state = stack.pop();
        match last_state {
            Some(ParseState::Object(mut pairs)) => {
                while let Some(token) = iter.next() {
                    match token {
                        Token::TString(key) => {
                            if iter.next().expect(UNEXPECTED_EOF) != Token::Colon {
                                panic!("Key-value pairs must have colon")
                            }
                            match iter.next().expect(UNEXPECTED_EOF) {
                                Token::OpenArray => {
                                    cur_key = Some(key);
                                    stack.push(ParseState::Object(pairs));
                                    stack.push(ParseState::Array(Vec::new()));
                                    break;
                                }
                                Token::OpenObject => {
                                    cur_key = Some(key);
                                    stack.push(ParseState::Object(pairs));
                                    stack.push(ParseState::Object(HashMap::new()));
                                    break;
                                }
                                val => { pairs.insert(key, parse_atom(val)); }
                            }
                        }
                        Token::Comma => continue,
                        Token::CloseObject => {
                            stack.push(ParseState::Value(JVal::JObject(pairs)));
                            break;
                        }
                        _ => panic!("Invalid Object Structure"),
                    }
                }
            }

            Some(ParseState::Array(mut array)) => {
                while let Some(token) = iter.next() {
                    match token {
                        Token::OpenArray => {
                            stack.push(ParseState::Array(array));
                            stack.push(ParseState::Array(Vec::new()));
                            break;
                        }
                        Token::OpenObject => {
                            stack.push(ParseState::Array(array));
                            stack.push(ParseState::Object(HashMap::new()));
                            break;
                        }
                        Token::Comma => continue,
                        Token::CloseArray => {
                            stack.push(ParseState::Value(JVal::JArray(array)));
                            break;
                        }
                        val => { array.push(parse_atom(val)); }
                    }
                }
            }

            Some(ParseState::Value(val)) => {
                match stack.pop(){
                    Some(ParseState::Array(mut array)) => {
                        array.push(val);
                        stack.push(ParseState::Array(array));
                    }
                    Some(ParseState::Object(mut pairs)) => {
                        match cur_key {
                            Some(ref key) => {
                                pairs.insert(key.clone(), val);
                                stack.push(ParseState::Object(pairs));
                            }
                            None => { panic!("Missing key for nested object/array") }
                        }
                    }
                    Some(ParseState::Value(_)) => {panic!("Value cannot follow a value")}
                    None => {return val}
                }
            }

            None => panic!("Invalid parse state"),
        }
    }

    panic!("Incorrect json structure")
}

fn parse_atom(token: Token) -> JVal {
    match token {
        Token::TString(val) => JVal::JString(val),
        Token::TNumber(val) => JVal::JNum(val),
        Token::TBool(val) => JVal::JBool(val),
        _ => panic!("Invalid value: {:?}", token),
    }
}
