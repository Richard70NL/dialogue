/************************************************************************************************/

use crate::constants::response::*;
use crate::log::LogMessage;
use crate::log::LogMessageType::*;
use crate::text::s;
use crate::text::Text::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;

/************************************************************************************************/

pub struct Session {
    stream: TcpStream,
    posting_allowed: bool,
}

/************************************************************************************************/

impl Session {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(stream: TcpStream) -> Session {
        Session {
            stream: stream,
            posting_allowed: false,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn run(&mut self) {
        let mut reader = BufReader::new(&self.stream);
        let mut writer = BufWriter::new(&self.stream);
        let mut line = String::new();
        let peer_addr = self.stream.peer_addr().unwrap();

        if self.posting_allowed {
            SERVICE_AVAILABLE_POSTING_ALLOWED.show_and_log(
                &mut writer,
                peer_addr,
                s(LogConnectionAccepted),
            );
        } else {
            SERVICE_AVAILABLE_POSTING_PROHIBITED.show_and_log(
                &mut writer,
                peer_addr,
                s(LogConnectionAccepted),
            );
        }

        'main_loop: loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(_len) => {
                    let command = parse_command_line(line.trim());
                    writeln(&mut writer, &format!("echo: {:?}", command));

                    if command.len() > 0 {
                        match command[0].to_lowercase().as_str() {
                            "quit" | "exit" | "logout" => {
                                write_response_and_log(
                                    &mut writer,
                                    peer_addr,
                                    205,
                                    "Connection closing.", // FIXME use text module
                                    &format!("received: {:?}", command), // FIXME use text module
                                );
                                break 'main_loop;
                            }
                            &_ => write_response_and_error(
                                &mut writer,
                                peer_addr,
                                500,
                                "Unknown command.", // FIXME use text module
                                &format!("received: {:?}", command), // FIXME use text module
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

fn write_response(writer: &mut BufWriter<&TcpStream>, response_code: u16, message: &str) {
    writeln(writer, &format!("{} {}", response_code, message));
}

/************************************************************************************************/

fn write_response_and_log(
    writer: &mut BufWriter<&TcpStream>,
    peer_addr: SocketAddr,
    response_code: u16,
    message: &str,
    log_message: &str,
) {
    write_response(writer, response_code, message);

    LogMessage::new(format!("{}; response: {}", log_message, message))
        .set_response_code(response_code)
        .set_client_addr(peer_addr)
        .show();
}

/************************************************************************************************/

fn write_response_and_error(
    writer: &mut BufWriter<&TcpStream>,
    peer_addr: SocketAddr,
    response_code: u16,
    message: &str,
    log_message: &str,
) {
    write_response(writer, response_code, message);

    LogMessage::new(format!("{}; response: {}", log_message, message))
        .set_type(Error)
        .set_response_code(response_code)
        .set_client_addr(peer_addr)
        .show();
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
