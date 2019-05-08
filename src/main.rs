/************************************************************************************************/

mod command;
mod constants;
mod database;
mod error;
mod log;
mod response;
mod server;
mod session;
mod text;
mod types;
mod util;
mod verbose;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::constants::cli::*;
use crate::constants::default::*;
use crate::database::Database;
use crate::error::DialogueError;
use crate::server::Server;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use crate::verbose::Verbose;
use clap::Arg;
use clap::SubCommand;
use std::io;
use std::net::SocketAddr;
use std::process::exit;
use std::str::FromStr;

/************************************************************************************************/

fn main() {
    exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            err.show();
            1
        }
    });
}

/************************************************************************************************/

fn run() -> Result<(), DialogueError> {
    let mut app = app_from_crate!()
        .subcommand(
            SubCommand::with_name(COMMAND_START_NAME)
                .about(s(CliStartAbout))
                .arg(
                    Arg::with_name(ARG_VERBOSE_NAME)
                        .short(ARG_VERBOSE_SHORT)
                        .long(ARG_VERBOSE_LONG)
                        .help(s(CliVerboseHelp)),
                )
                .arg(
                    Arg::with_name(ARG_ADDRESS_NAME)
                        .short(ARG_ADDRESS_SHORT)
                        .long(ARG_ADDRESS_LONG)
                        .help(s(CliAddressHelp))
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(ARG_DATABASE_URL_NAME)
                        .short(ARG_DATABASE_URL_SHORT)
                        .long(ARG_DATABASE_URL_LONG)
                        .help(s(CliDatabaseUrlHelp))
                        .default_value(DATA_BASE_URL),
                ),
        )
        .subcommand(
            SubCommand::with_name(COMMAND_STOP_NAME)
                .about(s(CliStopAbout))
                .arg(
                    Arg::with_name(ARG_VERBOSE_NAME)
                        .short(ARG_VERBOSE_SHORT)
                        .long(ARG_VERBOSE_LONG)
                        .help(s(CliVerboseHelp)),
                ),
        )
        .subcommand(
            SubCommand::with_name(COMMAND_INSTALL_NAME)
                .about(s(CliInstallAbout))
                .arg(
                    Arg::with_name(ARG_VERBOSE_NAME)
                        .short(ARG_VERBOSE_SHORT)
                        .long(ARG_VERBOSE_LONG)
                        .help(s(CliVerboseHelp)),
                )
                .arg(
                    Arg::with_name(ARG_DATABASE_URL_NAME)
                        .short(ARG_DATABASE_URL_SHORT)
                        .long(ARG_DATABASE_URL_LONG)
                        .help(s(CliDatabaseUrlHelp))
                        .default_value(DATA_BASE_URL),
                )
                .arg(
                    Arg::with_name(ARG_TEST_DATA_NAME)
                        .short(ARG_TEST_DATA_SHORT)
                        .long(ARG_TEST_DATA_LONG)
                        .help(s(CliTestDataHelp)),
                ),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand {
        None => {
            let mut out = io::stdout();
            match app.write_long_help(&mut out) {
                Ok(()) => {
                    println!();
                    Ok(())
                }
                Err(clap_err) => {
                    Err(DialogueError::new(clap_err.message).add(so(ErrorWriteLongHelp)))
                }
            }
        }
        Some(cmd) => {
            let mut verbose = Verbose::new();
            if cmd.matches.is_present(ARG_VERBOSE_NAME) {
                verbose.enable();
            }

            match cmd.name.as_str() {
                COMMAND_START_NAME => {
                    let address = cmd.matches.value_of(ARG_ADDRESS_NAME).unwrap(); // FIXME unwrap
                    let dburl = cmd.matches.value_of(ARG_DATABASE_URL_NAME).unwrap(); // FIXME unwrap
                    start_server(&verbose, address, dburl)?
                }
                COMMAND_STOP_NAME => stop_server(),
                COMMAND_INSTALL_NAME => {
                    let dburl = cmd.matches.value_of(ARG_DATABASE_URL_NAME).unwrap(); // FIXME unwrap
                    let test_data = cmd.matches.is_present(ARG_TEST_DATA_NAME);
                    install_database_schema(&verbose, dburl, test_data)?
                }
                &_ => Err(DialogueError::new(so(ErrorInvalidCommand)))?,
            }

            Ok(())
        }
    }
}

/************************************************************************************************/

fn start_server(verbose: &Verbose, address: &str, dburl: &str) -> Result<(), DialogueError> {
    verbose.println(s(LogInitializeServer));

    let address = SocketAddr::from_str(address).unwrap(); // FIXME unwrap
    let mut server = Server::new();

    server.set_binding_address(address);
    server.set_database_url(String::from(dburl));

    verbose.println(s(LogStartServer));
    server.start()?;

    verbose.println(s(LogDone));

    Ok(())
}

/************************************************************************************************/

fn stop_server() {
    // TODO: Implement stop server functionality.
    unimplemented!();
}

/************************************************************************************************/

fn install_database_schema(
    verbose: &Verbose,
    dburl: &str,
    test_data: bool,
) -> Result<(), DialogueError> {
    verbose.println(s(LogConnectingToDb));
    let database = Database::open(dburl)?;

    verbose.println(s(LogInstallingDbSchema));
    database.install()?;

    if test_data {
        verbose.println(s(LogInstallingTestData));
        database.install_test_data()?;
    }

    verbose.println(s(LogDone));

    Ok(())
}

/************************************************************************************************/
