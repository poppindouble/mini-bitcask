use kvs::constants as Constants;
use kvs::store_engine::KvStore;

use clap::{App, Arg, SubCommand};

fn main() {
    let arg_matches = App::new(Constants::APP_NAME)
        .version(Constants::APP_VERSION)
        .author("poppindouble <poppindouble@gmail.com>")
        .about(Constants::APP_DESCRIPTION)
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
            match arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_KEY) {
                Some(key) => println!("{:?}", key),
                None => unreachable!(),
            }
        }
        (Constants::SUBCOMMAND_GET, Some(arg_matches)) => {
            match arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_KEY) {
                Some(key) => println!("{:?}", key),
                None => unreachable!(),
            }
        }
        (Constants::SUBCOMMAND_REMOVE, Some(arg_matches)) => {
            match arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_KEY) {
                Some(key) => println!("{:?}", key),
                None => unimplemented!(),
            }
        }
        _ => unreachable!(),
    }
}
