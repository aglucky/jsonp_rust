use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() {
    let mut cmd = Command::cargo_bin("jsonp").unwrap();
    
    cmd.arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));
}

#[test]
fn can_read_file() {
    let mut cmd = Command::cargo_bin("jsonp").unwrap();

    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file.as_file_mut(), "Hello").unwrap();
    writeln!(temp_file.as_file_mut(), "World").unwrap();
    
    cmd.arg(temp_file.path())
        .assert()
        .success();
} 