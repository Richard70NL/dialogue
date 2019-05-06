/************************************************************************************************/

use crate::command::Command;
use crate::command::Command::*;
use crate::constants::env::*;
use crate::constants::response::*;
use crate::database::Database;
use crate::error::DialogueError;
use crate::group::Group;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use chrono::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;

/************************************************************************************************/

pub struct Session<'a> {
    stream: &'a TcpStream,
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
    posting_allowed: bool,
    database: Database,
    current_group: Option<Group>,
}

/************************************************************************************************/

impl<'a> Session<'a> {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(stream: &'a TcpStream, dburl: String) -> Session<'a> {
        Session {
            stream: stream,
            reader: BufReader::new(stream),
            writer: BufWriter::new(stream),
            posting_allowed: false,
            database: Database::open(&dburl).unwrap(), // FIXME unwrap
            current_group: None,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn run(&mut self) -> Result<(), DialogueError> {
        let mut line = String::new();
        let peer_addr = self.stream.peer_addr().unwrap();

        if self.posting_allowed {
            SERVICE_AVAILABLE_POSTING_ALLOWED.show_and_log(
                &mut self.writer,
                peer_addr,
                s(LogConnectionAccepted),
                &[],
            )?;
        } else {
            SERVICE_AVAILABLE_POSTING_PROHIBITED.show_and_log(
                &mut self.writer,
                peer_addr,
                s(LogConnectionAccepted),
                &[],
            )?;
        }

        self.writer.flush().unwrap(); // FIXME unwrap

        'main_loop: loop {
            // TODO: implement reader timeout!!!

            line.clear();
            match self.reader.read_line(&mut line) {
                Ok(_len) => {
                    let command = Command::parse(&line);
                    match &command {
                        Quit => {
                            CONNECTION_CLOSING.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                                &[],
                            )?;
                            break 'main_loop;
                        }
                        Capabilities => {
                            CAPABILITIES_LIST_FOLLOWS.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                                &[],
                            )?;
                            self.handle_capabilities()?;
                        }
                        Help => {
                            HELP_TEXT_FOLLOWS.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                                &[],
                            )?;
                            self.handle_help()?;
                        }
                        Date => {
                            let utc: DateTime<Utc> = Utc::now();
                            SERVER_DATE_TIME.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                                &[&utc.format("%Y%m%d%H%M%S").to_string()],
                            )?;
                        }
                        Group(group_str) => match self.database.get_group(group_str) {
                            Ok(group) => {
                                GROUP_SUCCESS.show_and_log_command(
                                    &mut self.writer,
                                    peer_addr,
                                    &command,
                                    &[
                                        &group.get_article_count().to_string(),
                                        &group.get_low_water_mark().to_string(),
                                        &group.get_high_water_mark().to_string(),
                                        &group.get_group_id(),
                                    ],
                                )?;
                                self.current_group = Some(group);
                            }
                            Err(error) => {
                                error.show();
                                NO_SUCH_GROUP.show_and_log_command(
                                    &mut self.writer,
                                    peer_addr,
                                    &command,
                                    &[],
                                )?;
                            }
                        },
                        Unknown(_) => UNKNOWN_COMMAND.show_and_log_command(
                            &mut self.writer,
                            peer_addr,
                            &command,
                            &[],
                        )?,
                        Invalid(_) => INVALID_COMMAND.show_and_log_command(
                            &mut self.writer,
                            peer_addr,
                            &command,
                            &[],
                        )?,
                    }

                    self.writer.flush().unwrap(); // FIXME unwrap
                }
                Err(e) => eprintln!("{}", e), // FIXME write propper error response
            }
        }

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    fn write(&mut self, s: &str) -> Result<(), DialogueError> {
        if let Err(e) = self.writer.write(s.as_bytes()) {
            Err(DialogueError::new(format!("{:?}", e)).add(so(ErrorWhileWriting)))
        } else {
            Ok(())
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn writeln(&mut self, line: &str) -> Result<(), DialogueError> {
        self.write(&format!("{}\n", line))
    }

    /*------------------------------------------------------------------------------------------*/

    fn handle_capabilities(&mut self) -> Result<(), DialogueError> {
        self.writeln("VERSION 2")?;
        self.writeln(&format!(
            "IMPLEMENTATION {} {}",
            CARGO_PKG_NAME, CARGO_PKG_VERSION
        ))?;
        self.writeln("READER")?;
        // self.writeln("IHAVE")?;
        // self.writeln("POST")?;
        // self.writeln("NEWNEWS")?;
        // self.writeln("HDR")?;
        // self.writeln("OVER")?;
        // self.writeln("LIST")?;
        // self.writeln("MODE-READER")?;
        self.writeln(".")?;

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    fn handle_help(&mut self) -> Result<(), DialogueError> {
        self.writeln("Known commands:")?;
        self.writeln("- QUIT")?;
        self.writeln("- CAPABILITIES")?;
        self.writeln("- HELP")?;
        self.writeln("- DATE")?;
        self.writeln("- GROUP")?;
        self.writeln(".")?;

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
