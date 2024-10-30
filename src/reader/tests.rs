use super::*;
use std::fs::write;
use tempfile::NamedTempFile;

fn create_temp_file(content: &str) -> (PathBuf, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    write(&path, content).unwrap();
    (path, temp_file)
}

#[test]
fn test_empty_file() {
    let (path, _guard) = create_temp_file("");
    let mut reader = JsonReader::new(path).unwrap();
    assert_eq!(reader.next(), None);
}

#[test]
fn test_single_line() {
    let (path, _guard) = create_temp_file("abc");
    let mut reader = JsonReader::new(path).unwrap();
    assert_eq!(reader.next(), Some('a'));
    assert_eq!(reader.next(), Some('b'));
    assert_eq!(reader.next(), Some('c'));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_multiple_lines() {
    let (path, _guard) = create_temp_file("ab\ncd");
    let mut reader = JsonReader::new(path).unwrap();
    assert_eq!(reader.next(), Some('a'));
    assert_eq!(reader.next(), Some('b'));
    assert_eq!(reader.next(), Some('\n'));
    assert_eq!(reader.next(), Some('c'));
    assert_eq!(reader.next(), Some('d'));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_peek() {
    let (path, _guard) = create_temp_file("abc");
    let mut reader = JsonReader::new(path).unwrap();
    
    assert_eq!(reader.peek(), Some('a'));
    assert_eq!(reader.peek(), Some('a'));
    assert_eq!(reader.next(), Some('a'));
    
    assert_eq!(reader.peek(), Some('b'));
    assert_eq!(reader.next(), Some('b'));
    assert_eq!(reader.next(), Some('c'));
    
    assert_eq!(reader.peek(), None);
    assert_eq!(reader.next(), None);
}

#[test]
fn test_invalid_path() {
    let result = JsonReader::new(PathBuf::from("/nonexistent/path"));
    assert!(result.is_err());
}
