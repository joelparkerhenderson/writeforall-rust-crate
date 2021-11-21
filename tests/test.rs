#[path = "util.rs"]
mod util;
use util::*;

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::io::Result;
use assertables::*;
use lazy_static::lazy_static;

#[test]
pub fn test_tutorial() {
    // Given
    let dir: PathBuf = TESTS_DIR.join("tutorial/");
    let tip = dir.join("tip.json");
    let input = dir.join("input.txt");
    let actual = dir.join("actual.txt");
    let expect = dir.join("actual.txt=expect.txt");
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--tip")
        .arg(tip.as_os_str())
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
