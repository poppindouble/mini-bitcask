use kvs::constants as Constants;

use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn cli_without_argument() {
    Command::cargo_bin(Constants::APP_NAME)
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn cli_get_version() {
    let expected_output = format!(
        "{app_name} {version}\n",
        app_name = Constants::APP_NAME,
        version = Constants::APP_VERSION
    );
    let actual_str_vec = Command::cargo_bin(Constants::APP_NAME)
        .unwrap()
        .arg(Constants::COMMAND_VERSION_FLAGS)
        .ok()
        .unwrap()
        .stdout;
    let actual_output = String::from_utf8(actual_str_vec).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn cli_get_command() {}

#[test]
fn cli_set_command() {}

#[test]
fn cli_rm_command() {}

#[test]
fn cli_invalid_get_command() {}

#[test]
fn cli_invalid_set_command() {}

#[test]
fn cli_invalid_rm_command() {}

#[test]
fn cli_invalid_subcommand() {}

#[test]
fn get_stored_value() {}

#[test]
fn overwrite_value() {}

#[test]
fn get_non_existent_value() {}

#[test]
fn remove_key() {}
