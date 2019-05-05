/************************************************************************************************/

use crate::command::Command;
use crate::error::DialogueError;
use crate::log::LogMessage;
use crate::log::LogMessageType::*;
use crate::text::so;
use crate::text::Text::*;
use crate::util::str_format;
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

    fn show(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        values: &[&str],
    ) -> Result<(), DialogueError> {
        if let Err(e) =
            writer.write(format!("{} {}\n", self.code, str_format(self.message, values)).as_bytes())
        {
            Err(DialogueError::new(format!("{:?}", e)).add(so(ErrorWhileWriting)))
        } else {
            Ok(())
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show_and_log(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        peer_addr: SocketAddr,
        log_message: &str,
        values: &[&str],
    ) -> Result<(), DialogueError> {
        LogMessage::new(format!(
            "{}; response: {}",
            log_message,
            str_format(self.message, values)
        ))
        .set_type(if self.code < 300 { Log } else { Error })
        .set_response_code(self.code)
        .set_client_addr(peer_addr)
        .show();

        self.show(writer, values)
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show_and_log_command(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        peer_addr: SocketAddr,
        command: &Command,
        values: &[&str],
    ) -> Result<(), DialogueError> {
        self.show_and_log(
            writer,
            peer_addr,
            &format!("received: {:?}", command),
            values,
        )
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
