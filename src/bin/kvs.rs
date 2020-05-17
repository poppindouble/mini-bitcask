use std::io::Result;

use kvs::constants as Constants;
use kvs::storage::bitcask::bitcask_engine::Bitcask;

use clap::{App, Arg, SubCommand};
use std::path::PathBuf;

fn main() -> Result<()> {
    let arg_matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_SET)
                .about(Constants::SUBCOMMAND_SET_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_VALUE)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_GET)
                .about(Constants::SUBCOMMAND_GET_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REMOVE)
                .about(Constants::SUBCOMMAND_REMOVE_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .get_matches();

    match arg_matches.subcommand() {
        (Constants::SUBCOMMAND_SET, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let value = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_VALUE)
                .expect(Constants::MISSING_VALUE_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut store_engine = Bitcask::open(&path)?;

            store_engine.set(key.as_bytes().to_vec(), value.as_bytes().to_vec())
        }
        (Constants::SUBCOMMAND_GET, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut store_engine = Bitcask::open(&path)?;

            match store_engine.get(key.as_bytes().to_vec())? {
                Some(res) => {
                    println!("{:?}", String::from_utf8(res));
                }
                None => {
                    println!("{:?}", Constants::MISSING_KEY_MESSAGE);
                }
            }

            return Ok(());
        }
        (Constants::SUBCOMMAND_REMOVE, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut store_engine = Bitcask::open(&path)?;

            store_engine.remove(key.as_bytes().to_vec())
        }
        _ => unreachable!(),
    }
}
