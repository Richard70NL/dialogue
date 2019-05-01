/************************************************************************************************/

use crate::log::LogMessage;
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

        if self.posting_allowed {
            write_response_and_log(
                &mut writer,
                self.stream.peer_addr().unwrap(),
                200,
                s(ResponseServiceAvailPostAllowed),
                s(LogConnectionAccepted),
            );
        } else {
            write_response_and_log(
                &mut writer,
                self.stream.peer_addr().unwrap(),
                201,
                s(ResponseServiceAvailPostProhibited),
                s(LogConnectionAccepted),
            );
        }

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(_len) => {
                    let command_line = line.trim();
                    writeln(&mut writer, &format!("echo: {}", command_line));
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

    LogMessage::new(format!("{} [{}]", log_message, message))
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
