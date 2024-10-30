use crate::lexer::{Token, TokenReader};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

const UNEXPECTED_EOF: &str = "Unexpected end of file while parsing JSON";

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

pub fn parse(iter: &mut TokenReader) -> Result<JVal, anyhow::Error> {
    let mut stack: Vec<ParseState> = Vec::new();
    let mut key_stack: Vec<String> = Vec::new();

    match iter.next()
        .ok_or_else(|| anyhow::anyhow!("Empty input"))??
    {
        Token::OpenObject => stack.push(ParseState::Object(HashMap::new())),
        Token::OpenArray => stack.push(ParseState::Array(Vec::new())),
        _ => return Err(anyhow::anyhow!("Invalid JSON: Document must start with either '{{' or '['"))
    }

    while !stack.is_empty() {
        let last_state = stack.pop();
        match last_state {
            Some(ParseState::Object(mut pairs)) => {
                while let Some(token) = iter.next() {
                    match token? {
                        Token::TString(key) => {
                            let colon = iter.next()
                                .ok_or_else(|| anyhow::anyhow!(UNEXPECTED_EOF))??;
                            if colon != Token::Colon {
                                return Err(anyhow::anyhow!("Invalid JSON object: Expected ':' after key '{}'", key));
                            }
                            
                            let val = iter.next()
                                .ok_or_else(|| anyhow::anyhow!(UNEXPECTED_EOF))??;
                            match val {
                                Token::OpenArray => {
                                    key_stack.push(key);
                                    stack.push(ParseState::Object(pairs));
                                    stack.push(ParseState::Array(Vec::new()));
                                    break;
                                }
                                Token::OpenObject => {
                                    key_stack.push(key);
                                    stack.push(ParseState::Object(pairs));
                                    stack.push(ParseState::Object(HashMap::new()));
                                    break;
                                }
                                val => { pairs.insert(key, parse_atom(val)?); }
                            }
                        }
                        Token::Comma => continue,
                        Token::CloseObject => {
                            stack.push(ParseState::Value(JVal::JObject(pairs)));
                            break;
                        }
                        _ => return Err(anyhow::anyhow!("Invalid JSON object structure: Expected string key, '}}', or ','"))
                    }
                }
            }

            Some(ParseState::Array(mut array)) => {
                while let Some(token) = iter.next() {
                    match token? {
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
                        val => { array.push(parse_atom(val)?); }
                    }
                }
            }

            Some(ParseState::Value(val)) => {
                match stack.pop() {
                    Some(ParseState::Array(mut array)) => {
                        array.push(val);
                        stack.push(ParseState::Array(array));
                    }
                    Some(ParseState::Object(mut pairs)) => {
                        match key_stack.pop() {
                            Some(key) => {
                                pairs.insert(key, val);
                                stack.push(ParseState::Object(pairs));
                            }
                            None => return Err(anyhow::anyhow!("Invalid JSON structure: Missing key for nested object/array"))
                        }
                    }
                    Some(ParseState::Value(_)) => return Err(anyhow::anyhow!("Invalid JSON structure: Cannot have consecutive values without separators")),
                    None => return Ok(val)
                }
            }

            None => return Err(anyhow::anyhow!("Internal parser error: Invalid parse state")),
        }
    }

    Err(anyhow::anyhow!("Invalid JSON structure: Unclosed object or array"))
}

fn parse_atom(token: Token) -> Result<JVal, anyhow::Error> {
    match token {
        Token::TString(val) => Ok(JVal::JString(val)),
        Token::TNumber(val) => Ok(JVal::JNum(val)),
        Token::TBool(val) => Ok(JVal::JBool(val)),
        _ => Err(anyhow::anyhow!("Invalid JSON value: Expected string, number, or boolean, got {:?}", token))
    }
}
