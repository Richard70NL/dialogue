/************************************************************************************************/

use crate::log::LogMessage;
use crate::text::s;
use crate::text::Text::*;
use std::io::Write;
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
        //let reader = BufReader::new(self.stream);

        if self.posting_allowed {
            self.write_response_and_log(
                200,
                s(ResponseServiceAvailPostAllowed),
                s(LogConnectionAccepted),
            );
        } else {
            self.write_response_and_log(
                201,
                s(ResponseServiceAvailPostProhibited),
                s(LogConnectionAccepted),
            );
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn write_response(&mut self, response_code: u16, message: &str) {
        self.writeln(&format!("{} {}", response_code, message));
    }

    /*------------------------------------------------------------------------------------------*/

    fn write_response_and_log(&mut self, response_code: u16, message: &str, log_message: &str) {
        self.write_response(response_code, message);

        LogMessage::new(format!("{} [{}]", log_message, message))
            .set_response_code(response_code)
            .set_client_addr(self.stream.peer_addr().unwrap())
            .show();
    }

    /*------------------------------------------------------------------------------------------*/

    fn writeln(&mut self, line: &str) {
        self.write(&format!("{}\n", line));
    }

    /*------------------------------------------------------------------------------------------*/

    fn write(&mut self, s: &str) {
        let _ = self.stream.write(String::from(s).as_bytes());
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
