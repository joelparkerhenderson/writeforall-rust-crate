use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

#[allow(dead_code)]
fn assert_str_contains(outer: &str, inner: &str) {
    assert!(
        outer.contains(inner), 
        "outer: {:?}\n inner: {}\n", &outer, &inner
    );
}

fn assert_command_stdout_contains(command_name: &str, command_args: &[&str], target: &str) {
    let output = Command::new(command_name)
    .args(command_args)
    .output()
    .expect("failure");
    let actual = ::std::str::from_utf8(&output.stdout)
    .unwrap()
    .to_string();
    assert!(
        actual.contains(target), 
        "command: {:?}\nargs: {:?}\nactual: {:?}\ntarget: {}\n", &command_name, &command_args, &actual, &target
    );
}

/// High-priority CLAP command args tests

// Test that the special argument `--test` is working.
//
// This test must succeed in order for any of the rest of the tests here to
// succeed, because the `--test` argument turns on the runtime output to stdout,
// which includes a typical print debug of the entire `args` structure.
//
#[test]
fn test_clap_test() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#"Args { "#);
}

// Test that the special argument `--verbose` is working.
//
// This test must succeed in order for any of the rest of the tests here to
// show diagnostics, because the `--verbose` argument turns on logging output,
// which can include debugging messages, warnings, errors, and so on.
//
#[test]
fn test_clap_verbose() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#" log_level: None"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-v"], r#" log_level: Some(Error)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "--verbose"], r#" log_level: Some(Error)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose"], r#" log_level: Some(Warn)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Info)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Debug)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Trace)"#);
}

/// Normal-priority CLAP command args tests

#[test]
fn test_clap_input() {
    let s1 = "alpha";
    let s2 = "bravo";
    let s3 = "charlie";
    let s4 = "delta";
    let target = format!(" input_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);
    // Test short `-i` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-i", &s1, &s2, "-i", &s3, &s4], 
        &target
    );
    // Test long `--input` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--input", &s1, &s2, "--input", &s3, &s4], 
        &target
    );
    // Test alias `--inputs` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--inputs", &s1, &s2, "--inputs", &s3, &s4], 
        &target
    );
}

#[test]
fn test_clap_set() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--set", "alpha", "bravo", "--set", "charlie", "delta"], 
        r#" settings: Some({"alpha": "bravo", "charlie": "delta"})"#
    );
}

#[test]
fn test_clap_tip() {
    let dir = "tip_list_pathable_string";
    let s1 = format!("{}/{}", &dir, "a/**/*");
    let s2 = format!("{}/{}", &dir, "b/**/*");
    let s3 = format!("{}/{}", &dir, "c/**/*");
    let s4 = format!("{}/{}", &dir, "d/**/*");
    let target = format!(" tip_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-t", &s1, &s2, "-t", &s3, &s4], 
        &target
    );
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--tip", &s1, &s2, "--tip", &s3, &s4], 
        &target
    );
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--tips", &s1, &s2, "--tips", &s3, &s4], 
        &target
    );
}


