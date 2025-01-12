use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("claude").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"));
}

#[test]
fn test_single_message_mode() {
    let mut cmd = Command::cargo_bin("claude").unwrap();
    cmd.arg("--model")
        .arg("claude-3-sonnet")
        .arg("What is 2+2?")
        .assert()
        .success();
}

#[test]
fn test_output_format() {
    let mut cmd = Command::cargo_bin("claude").unwrap();
    cmd.arg("--format")
        .arg("json")
        .arg("List three colors")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"));
}

#[test]
fn test_config_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    
    let mut cmd = Command::cargo_bin("claude-config").unwrap();
    cmd.env("CLAUDE_CONFIG_PATH", config_path.to_str().unwrap())
        .arg("--model")
        .arg("claude-3-opus")
        .assert()
        .success();
        
    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("claude-3-opus"));
}

#[test]
fn test_repl_commands() {
    let mut cmd = Command::cargo_bin("claude").unwrap();
    cmd.write_stdin(":help\n:q\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available Commands:"));
}