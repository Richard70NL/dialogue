/************************************************************************************************/

mod constants;
mod error;
mod text;
mod verbose;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::constants::*;
use crate::error::DialogueError;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use crate::verbose::Verbose;
use clap::Arg;
use clap::SubCommand;
use std::io;
use std::process::exit;

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
    let mut app = app_from_crate!().subcommand(
        SubCommand::with_name(COMMAND_LISTEN_NAME)
            .about(s(CliListenAbout))
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

            if cmd.name == COMMAND_LISTEN_NAME {
                unimplemented!();
            }

            Ok(())
        }
    }
}

/************************************************************************************************/
