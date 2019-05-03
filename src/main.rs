/************************************************************************************************/

mod constants;
mod error;
mod log;
mod response;
mod server;
mod session;
mod text;
mod verbose;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::constants::cli::*;
use crate::constants::default::*;
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
                    Err(DialogueError::new(so(ErrorWriteLongHelp)).add(clap_err.message))
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
                COMMAND_INSTALL_NAME => install_database_schema(),
                &_ => Err(DialogueError::new(so(ErrorInvalidCommand)))?,
            }

            Ok(())
        }
    }
}

/************************************************************************************************/

fn start_server(verbose: &Verbose, address: &str, dburl: &str) -> Result<(), DialogueError> {
    verbose.println("Initializing the server."); // FIXME use text module
    let mut server = Server::new();
    server.set_binding_address(SocketAddr::from_str(address).unwrap()); // FIXME unwrap
    server.set_database_url(String::from(dburl));

    verbose.println("Start the server."); // FIXME use text module
    server.start()?;

    verbose.println("Done!"); // FIXME use text module

    Ok(())
}

/************************************************************************************************/

fn stop_server() {
    unimplemented!();
}

/************************************************************************************************/

fn install_database_schema() {
    unimplemented!();
}

/************************************************************************************************/
