/************************************************************************************************/

use crate::constants::*;
use crate::error::DialogueError;
use crate::log::LogMessage;
use crate::log::LogMessageType::*;
use crate::text::sr;
use crate::text::Text::*;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

/************************************************************************************************/

#[derive(Debug)]
pub struct Server {
    address: IpAddr, // FIXME join address and port into one SocketAddr
    port: u16,
    database_url: String,
}

/************************************************************************************************/

impl Server {
    /*------------------------------------------------------------------------------------------*/

    pub fn new() -> Server {
        Server {
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), // FIXME make address optional, don't assume 0.0.0.0 is correct
            port: 119,
            database_url: DEFAULT_DATA_BASE_URL.to_string(),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_binding_address(&mut self, address: IpAddr) {
        self.address = address;
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_binding_port(&mut self, port: u16) {
        self.port = port;
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_database_url(&mut self, dburl: String) {
        self.database_url = dburl
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn start(&self) -> Result<(), DialogueError> {
        let bind_addr = SocketAddr::from((self.address, self.port));
        let listener = TcpListener::bind(bind_addr).or_else(|e| {
            Err(DialogueError::new(format!("{:?}", e)).add(sr(
                ErrorBindingListener,
                &[&self.address.to_string(), &self.port.to_string()],
            )))
        })?;

        LogMessage::new(100, String::from("Server starts listening.")).show(); // FIXME use text module

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    if self.accept_by_address(&addr) {
                        self.handle_connection(stream)
                    } else {
                        LogMessage::new(400, format!("rejected connection from: {:?}", addr)) // FIXME use text module
                            .set_type(Error)
                            .show();
                    }
                }
                Err(e) => {
                    LogMessage::new(500, format!("couldn't get client: {:?}", e)) // FIXME use text module
                        .set_type(Error)
                        .show()
                }
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn accept_by_address(&self, _addr: &SocketAddr) -> bool {
        // TODO: implement black/white list functionality here

        true
    }

    /*------------------------------------------------------------------------------------------*/

    fn handle_connection(&self, mut stream: TcpStream) {
        let _handler = spawn(move || {
            LogMessage::new(
                100,
                format!(
                    "accepted connection from: {:?}",
                    stream.peer_addr().unwrap()
                ), // FIXME use text module
            )
            .show();

            let _ = stream.write(b"CONNECTED and CLOSED again...\n");
        });
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
