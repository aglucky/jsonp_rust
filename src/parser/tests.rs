use super::*;
use crate::{lexer::TokenReader, reader::JsonReader};

fn parse_str(input: &str) -> JVal {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(&temp_file, input).unwrap();
    let file_reader = JsonReader::new(temp_file.path().to_path_buf()).unwrap();
    let mut reader = TokenReader::new(file_reader);
    parse(&mut reader).unwrap()
}

#[test]
fn test_parse_simple_object() {
    let input = r#"{"name": "John", "age": 30}"#;
    let result = parse_str(input);
    
    if let JVal::JObject(map) = result {
        assert_eq!(map.get("name"), Some(&JVal::JString("John".to_string())));
        assert_eq!(map.get("age"), Some(&JVal::JNum(30.0)));
    } else {
        panic!("Expected JObject");
    }
}

#[test]
fn test_parse_null_keyt() {
    let input = r#"{"name": null, "age": 30}"#;
    let result = parse_str(input);
    
    if let JVal::JObject(map) = result {
        assert_eq!(map.get("name"), Some(&JVal::JNull));
        assert_eq!(map.get("age"), Some(&JVal::JNum(30.0)));
    } else {
        panic!("Expected JObject");
    }
}

#[test]
fn test_parse_simple_array() {
    let input = r#"[1, 2, 3, "test"]"#;
    let result = parse_str(input);
    
    if let JVal::JArray(arr) = result {
        assert_eq!(arr[0], JVal::JNum(1.0));
        assert_eq!(arr[1], JVal::JNum(2.0));
        assert_eq!(arr[2], JVal::JNum(3.0));
        assert_eq!(arr[3], JVal::JString("test".to_string()));
    } else {
        panic!("Expected JArray");
    }
}

#[test]
fn test_parse_nested_object() {
    let input = r#"{"user": {"name": "John", "active": true}}"#;
    let result = parse_str(input);
    
    if let JVal::JObject(map) = result {
        if let Some(JVal::JObject(inner)) = map.get("user") {
            assert_eq!(inner.get("name"), Some(&JVal::JString("John".to_string())));
            assert_eq!(inner.get("active"), Some(&JVal::JBool(true)));
        } else {
            panic!("Expected nested object");
        }
    } else {
        panic!("Expected JObject");
    }
}

#[test]
fn test_parse_nested_array() {
    let input = r#"[1, [2, 3], 4]"#;
    let result = parse_str(input);
    
    if let JVal::JArray(arr) = result {
        assert_eq!(arr[0], JVal::JNum(1.0));
        if let JVal::JArray(inner) = &arr[1] {
            assert_eq!(inner[0], JVal::JNum(2.0));
            assert_eq!(inner[1], JVal::JNum(3.0));
        } else {
            panic!("Expected nested array");
        }
        assert_eq!(arr[2], JVal::JNum(4.0));
    } else {
        panic!("Expected JArray");
    }
}

#[test]
#[should_panic(expected = "Document must start with either '{' or '['")]
fn test_invalid_json_start() {
    parse_str(r#""invalid""#);
}

#[test]
#[should_panic(expected = "Invalid JSON object: Expected ':' after key")]
fn test_missing_colon() {
    parse_str(r#"{"key" "value"}"#);
}

#[test]
#[should_panic(expected = "Invalid JSON structure: Unclosed object or array")]
fn test_unclosed_object() {
    parse_str(r#"{"key": "value""#);
}

#[test]
fn test_complex_nested_structure() {
    let input = r#"
    {
        "name": "John",
        "details": {
            "age": 30,
            "hobbies": ["reading", "coding"],
            "address": {
                "city": "New York",
                "zip": 10001
            }
        },
        "active": true
    }"#;
    
    let result = parse_str(input);
    
    if let JVal::JObject(map) = result {
        assert_eq!(map.get("name"), Some(&JVal::JString("John".to_string())));
        assert_eq!(map.get("active"), Some(&JVal::JBool(true)));
        
        if let Some(JVal::JObject(details)) = map.get("details") {
            assert_eq!(details.get("age"), Some(&JVal::JNum(30.0)));
            
            if let Some(JVal::JArray(hobbies)) = details.get("hobbies") {
                assert_eq!(hobbies[0], JVal::JString("reading".to_string()));
                assert_eq!(hobbies[1], JVal::JString("coding".to_string()));
            } else {
                panic!("Expected hobbies array");
            }
            
            if let Some(JVal::JObject(address)) = details.get("address") {
                assert_eq!(address.get("city"), Some(&JVal::JString("New York".to_string())));
                assert_eq!(address.get("zip"), Some(&JVal::JNum(10001.0)));
            } else {
                panic!("Expected address object");
            }
        } else {
            panic!("Expected details object");
        }
    } else {
        panic!("Expected JObject");
    }
}
