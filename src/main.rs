/************************************************************************************************/

mod constants;
mod error;
mod server;
mod text;
mod verbose;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::constants::*;
use crate::error::DialogueError;
use crate::server::Server;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use crate::verbose::Verbose;
use clap::Arg;
use clap::SubCommand;
use std::io;
use std::net::IpAddr;
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
                        .default_value(DEFAULT_ADDRESS),
                )
                .arg(
                    Arg::with_name(ARG_PORT_NAME)
                        .short(ARG_PORT_SHORT)
                        .long(ARG_PORT_LONG)
                        .help(s(CliPortHelp))
                        .default_value(DEFAULT_PORT),
                )
                .arg(
                    Arg::with_name(ARG_DATABASE_URL_NAME)
                        .short(ARG_DATABASE_URL_SHORT)
                        .long(ARG_DATABASE_URL_LONG)
                        .help(s(CliDatabaseUrlHelp))
                        .default_value(DEFAULT_DATA_BASE_URL),
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
                    let port = cmd.matches.value_of(ARG_PORT_NAME).unwrap(); // FIXME unwrap
                    let dburl = cmd.matches.value_of(ARG_DATABASE_URL_NAME).unwrap(); // FIXME unwrap
                    start_server(&verbose, address, port, dburl)
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

fn start_server(verbose: &Verbose, address: &str, port: &str, dburl: &str) {
    verbose.println("Initializing the server.");

    let mut server = Server::new();

    server.set_binding_address(IpAddr::from_str(address).unwrap()); // FIXME unwrap
    server.set_binding_port(port.parse::<u16>().unwrap()); // FIXME unwrap
    server.set_database_url(String::from(dburl));

    verbose.println("Start the server.");
    server.start();

    verbose.println("Done!");
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
