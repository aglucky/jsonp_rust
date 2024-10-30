use crate::lexer::{Token, TokenReader};
use crate::reader::JsonReader;
use tempfile;

fn tokenize(input: &str) -> Vec<Token> {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(&temp_file, input).unwrap();
    let reader = JsonReader::new(temp_file.path().to_path_buf()).unwrap();
    TokenReader::new(reader).collect::<Result<Vec<Token>, _>>().unwrap()
}

#[test]
fn test_basic_tokens() {
    let tokens = tokenize("{}[]:,");

    assert_eq!(
        tokens,
        vec![
            Token::OpenObject,
            Token::CloseObject,
            Token::OpenArray,
            Token::CloseArray,
            Token::Colon,
            Token::Comma,
        ]
    );
}

#[test]
fn test_string_tokens() {
    let tokens = tokenize(r#""hello" "world\n" "escaped\"quote""#);
    
    assert_eq!(
        tokens,
        vec![
            Token::TString("hello".to_string()),
            Token::TString("world\n".to_string()),
            Token::TString("escaped\"quote".to_string()),
        ]
    );
}

#[test]
fn test_number_tokens() {
    let tokens = tokenize("123 -456.789 0.123 -0.0");
    
    assert_eq!(
        tokens,
        vec![
            Token::TNumber(123.0),
            Token::TNumber(-456.789),
            Token::TNumber(0.123),
            Token::TNumber(-0.0),
        ]
    );
}

#[test]
fn test_literal_tokens() {
    let tokens = tokenize("true false null");
    
    assert_eq!(
        tokens,
        vec![
            Token::TBool(true),
            Token::TBool(false),
            Token::TNull,
        ]
    );
}

#[test]
fn test_whitespace_handling() {
    let tokens = tokenize(" \n\t{ \r\n} \t");
    
    assert_eq!(
        tokens,
        vec![
            Token::OpenObject,
            Token::CloseObject,
        ]
    );
}

#[test]
fn test_complex_structure() {
    let tokens = tokenize(r#"{"key": [1, true, "value"]}"#);
    
    assert_eq!(
        tokens,
        vec![
            Token::OpenObject,
            Token::TString("key".to_string()),
            Token::Colon,
            Token::OpenArray,
            Token::TNumber(1.0),
            Token::Comma,
            Token::TBool(true),
            Token::Comma,
            Token::TString("value".to_string()),
            Token::CloseArray,
            Token::CloseObject,
        ]
    );
}
