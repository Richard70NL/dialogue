/************************************************************************************************/

use crate::command::Command;
use crate::log::LogMessage;
use crate::log::LogMessageType::*;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;

/************************************************************************************************/

pub struct Response {
    pub code: u16,
    pub message: &'static str,
}

/************************************************************************************************/

impl Response {
    /*------------------------------------------------------------------------------------------*/

    fn show(&self, writer: &mut BufWriter<&TcpStream>) {
        writer
            .write(format!("{} {}\n", self.code, self.message).as_bytes())
            .unwrap(); // FIXME unwrap
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show_and_log(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        peer_addr: SocketAddr,
        log_message: &str,
    ) {
        self.show(writer);

        LogMessage::new(format!("{}; response: {}", log_message, self.message))
            .set_type(if self.code < 300 { Log } else { Error })
            .set_response_code(self.code)
            .set_client_addr(peer_addr)
            .show();
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show_and_log_command(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        peer_addr: SocketAddr,
        command: &Command,
    ) {
        self.show_and_log(writer, peer_addr, &format!("received: {:?}", command))
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
