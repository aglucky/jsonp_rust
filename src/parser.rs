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
    JNull,
}

#[derive(Debug)]
enum ParseState {
    Object(HashMap<String, JVal>),
    Array(Vec<JVal>),
    Value(JVal),
}

pub fn parse(iter: &mut TokenReader) -> Result<JVal, anyhow::Error> {
    let mut state_stack: Vec<ParseState> = Vec::new();
    let mut key_stack: Vec<String> = Vec::new();
    let mut comma_stack: Vec<bool> = Vec::new();

    match iter
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty input"))??
    {
        Token::OpenObject => state_stack.push(ParseState::Object(HashMap::new())),
        Token::OpenArray => state_stack.push(ParseState::Array(Vec::new())),
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid JSON: Document must start with either '{{' or '['"
            ))
        }
    }

    while !state_stack.is_empty() {
        let last_state = state_stack.pop();
        match last_state {
            Some(ParseState::Object(pairs)) => {
                parse_object(
                    iter,
                    &mut state_stack,
                    &mut key_stack,
                    &mut comma_stack,
                    pairs,
                )?;
            }
            Some(ParseState::Array(array)) => {
                parse_array(iter, &mut state_stack, &mut comma_stack, array)?;
            }
            Some(ParseState::Value(val)) => {
                match parse_value(&mut state_stack, &mut key_stack, val)? {
                    Some(final_value) => return Ok(final_value),
                    None => continue,
                }
            }
            None => {
                return Err(anyhow::anyhow!(
                    "Internal parser error: Invalid parse state"
                ))
            }
        }
    }

    Err(anyhow::anyhow!(
        "Invalid JSON structure: Unclosed object or array"
    ))
}

fn parse_object(
    iter: &mut TokenReader,
    state_stack: &mut Vec<ParseState>,
    key_stack: &mut Vec<String>,
    comma_stack: &mut Vec<bool>,
    mut pairs: HashMap<String, JVal>,
) -> Result<(), anyhow::Error> {
    while let Some(token) = iter.next() {
        match token? {
            Token::TString(key) => {
                let colon = iter
                    .next()
                    .ok_or_else(|| anyhow::anyhow!(UNEXPECTED_EOF))??;
                if colon != Token::Colon {
                    return Err(anyhow::anyhow!(
                        "Invalid JSON object: Expected ':' after key '{}'",
                        key
                    ));
                }

                let val = iter
                    .next()
                    .ok_or_else(|| anyhow::anyhow!(UNEXPECTED_EOF))??;
                match val {
                    Token::OpenArray => {
                        check_comma(pairs.len(), comma_stack)?;
                        key_stack.push(key);
                        state_stack.push(ParseState::Object(pairs));
                        state_stack.push(ParseState::Array(Vec::new()));
                        break;
                    }
                    Token::OpenObject => {
                        check_comma(pairs.len(), comma_stack)?;
                        key_stack.push(key);
                        state_stack.push(ParseState::Object(pairs));
                        state_stack.push(ParseState::Object(HashMap::new()));
                        break;
                    }
                    val => {
                        check_comma(pairs.len(), comma_stack)?;
                        pairs.insert(key, parse_atom(val)?);
                    }
                }
            }
            Token::Comma => comma_stack.push(true),
            Token::CloseObject => {
                state_stack.push(ParseState::Value(JVal::JObject(pairs)));
                break;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid JSON object structure: Expected string key, '}}', or ','"
                ))
            }
        }
    }
    Ok(())
}

fn parse_array(
    iter: &mut TokenReader,
    state_stack: &mut Vec<ParseState>,
    comma_stack: &mut Vec<bool>,
    mut array: Vec<JVal>,
) -> Result<(), anyhow::Error> {
    while let Some(token) = iter.next() {
        match token? {
            Token::OpenArray => {
                check_comma(array.len(), comma_stack)?;
                state_stack.push(ParseState::Array(array));
                state_stack.push(ParseState::Array(Vec::new()));
                break;
            }
            Token::OpenObject => {
                check_comma(array.len(), comma_stack)?;
                state_stack.push(ParseState::Array(array));
                state_stack.push(ParseState::Object(HashMap::new()));
                break;
            }
            Token::Comma => comma_stack.push(true),
            Token::CloseArray => {
                state_stack.push(ParseState::Value(JVal::JArray(array)));
                break;
            }
            val => {
                check_comma(array.len(), comma_stack)?;
                array.push(parse_atom(val)?);
            }
        }
    }
    Ok(())
}

fn parse_value(
    state_stack: &mut Vec<ParseState>,
    key_stack: &mut Vec<String>,
    value: JVal,
) -> Result<Option<JVal>, anyhow::Error> {
    match state_stack.pop() {
        Some(ParseState::Array(mut array)) => {
            array.push(value);
            state_stack.push(ParseState::Array(array));
        }
        Some(ParseState::Object(mut pairs)) => match key_stack.pop() {
            Some(key) => {
                pairs.insert(key, value);
                state_stack.push(ParseState::Object(pairs));
            }
            None => {
                return Err(anyhow::anyhow!(
                    "Invalid JSON structure: Missing key for nested object/array"
                ))
            }
        },
        Some(ParseState::Value(_)) => {
            return Err(anyhow::anyhow!(
                "Invalid JSON structure: Cannot have consecutive values without separators"
            ))
        }
        None => return Ok(Some(value)),
    }
    Ok(None)
}

fn parse_atom(token: Token) -> Result<JVal, anyhow::Error> {
    match token {
        Token::TString(val) => Ok(JVal::JString(val)),
        Token::TNumber(val) => Ok(JVal::JNum(val)),
        Token::TBool(val) => Ok(JVal::JBool(val)),
        Token::TNull => Ok(JVal::JNull),
        _ => Err(anyhow::anyhow!(
            "Invalid JSON value: Expected string, number, or boolean, got {:?}",
            token
        )),
    }
}

fn check_comma(length: usize, comma_stack: &mut Vec<bool>) -> Result<(), anyhow::Error> {
    if length > 0 && comma_stack.pop() != Some(true) {
        return Err(anyhow::anyhow!("Missing comma between elements"));
    }
    Ok(())
}
