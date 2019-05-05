/************************************************************************************************/

use crate::command::Command;
use crate::command::Command::*;
use crate::constants::env::*;
use crate::constants::response::*;
use crate::error::DialogueError;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
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
}

/************************************************************************************************/

impl<'a> Session<'a> {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(stream: &TcpStream) -> Session {
        Session {
            stream: stream,
            reader: BufReader::new(stream),
            writer: BufWriter::new(stream),
            posting_allowed: false,
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
            )?;
        } else {
            SERVICE_AVAILABLE_POSTING_PROHIBITED.show_and_log(
                &mut self.writer,
                peer_addr,
                s(LogConnectionAccepted),
            )?;
        }

        'main_loop: loop {
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
                            )?;
                            break 'main_loop;
                        }
                        Capabilities => {
                            CAPABILITIES_LIST_FOLLOWS.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                            )?;
                            self.handle_capabilities()?;
                        }
                        Unknown(_) => UNKNOWN_COMMAND.show_and_log_command(
                            &mut self.writer,
                            peer_addr,
                            &command,
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
}

/************************************************************************************************/
