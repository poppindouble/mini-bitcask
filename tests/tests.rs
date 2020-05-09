use kvs::constants as Constants;

use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn cli_without_argument() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn cli_get_version() {
    let expected_output = format!(
        "{app_name} {version}\n",
        app_name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION")
    );
    let actual_str_vec = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg(Constants::COMMAND_VERSION_FLAGS)
        .ok()
        .unwrap()
        .stdout;
    let actual_output = String::from_utf8(actual_str_vec).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn cli_invalid_get_command() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_GET])
        .assert()
        .failure();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_GET, "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set_command() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_SET])
        .assert()
        .failure();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_SET, "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_SET, "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm_command() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_REMOVE])
        .assert()
        .failure();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[Constants::SUBCOMMAND_REMOVE, "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}

#[test]
#[ignore]
fn cli_get_command() {}

#[test]
#[ignore]
fn cli_set_command() {}

#[test]
#[ignore]
fn cli_rm_command() {}

#[test]
#[ignore]
fn get_stored_value() {}

#[test]
#[ignore]
fn overwrite_value() {}

#[test]
#[ignore]
fn get_non_existent_value() {}

#[test]
#[ignore]
fn remove_key() {}
