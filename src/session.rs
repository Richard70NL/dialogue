/************************************************************************************************/

use crate::constants::response::*;
use crate::text::s;
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

    pub fn run(&mut self) {
        let mut line = String::new();
        let peer_addr = self.stream.peer_addr().unwrap();

        if self.posting_allowed {
            SERVICE_AVAILABLE_POSTING_ALLOWED.show_and_log(
                &mut self.writer,
                peer_addr,
                s(LogConnectionAccepted),
            );
        } else {
            SERVICE_AVAILABLE_POSTING_PROHIBITED.show_and_log(
                &mut self.writer,
                peer_addr,
                s(LogConnectionAccepted),
            );
        }

        'main_loop: loop {
            line.clear();
            match self.reader.read_line(&mut line) {
                Ok(_len) => {
                    let command = parse_command_line(line.trim());
                    writeln(&mut self.writer, &format!("echo: {:?}", command));

                    if command.len() > 0 {
                        match command[0].to_lowercase().as_str() {
                            "quit" | "exit" | "logout" => {
                                CONNECTION_CLOSING.show_and_log_command(
                                    &mut self.writer,
                                    peer_addr,
                                    &command,
                                );
                                break 'main_loop;
                            }
                            &_ => UNKNOWN_COMMAND.show_and_log_command(
                                &mut self.writer,
                                peer_addr,
                                &command,
                            ),
                        }
                    }
                }
                Err(e) => eprintln!("{}", e), // FIXME write propper error response
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

fn writeln(writer: &mut BufWriter<&TcpStream>, line: &str) {
    write(writer, &format!("{}\n", line));
}

/************************************************************************************************/

fn write(writer: &mut BufWriter<&TcpStream>, s: &str) {
    writer.write(s.as_bytes()).unwrap();
    writer.flush().unwrap();
}

/************************************************************************************************/

fn parse_command_line(command_line: &str) -> Vec<String> {
    let command_line_string = String::from(command_line);
    let iter = command_line_string.split_ascii_whitespace();
    let command: Vec<String> = iter.map(|s| String::from(s)).collect();

    command
}

/************************************************************************************************/
