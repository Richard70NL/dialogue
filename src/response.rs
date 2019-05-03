/************************************************************************************************/

use crate::log::LogMessage;
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

    pub fn show(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        peer_addr: SocketAddr,
        log_message: &str,
    ) {
        self.write_response(writer);

        LogMessage::new(format!("{}; response: {}", log_message, self.message))
            .set_response_code(self.code)
            .set_client_addr(peer_addr)
            .show();
    }

    /*------------------------------------------------------------------------------------------*/

    fn write_response(&self, writer: &mut BufWriter<&TcpStream>) {
        let m = format!("{} {}\n", self.code, self.message);

        writer.write(m.as_bytes()).unwrap();
        writer.flush().unwrap();
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
