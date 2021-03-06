/************************************************************************************************/

use crate::constants::default::*;
use crate::error::DialogueError;
use crate::log::LogMessage;
use crate::log::LogMessageType::*;
use crate::session::Session;
use crate::text::sr;
use crate::text::Text::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

/************************************************************************************************/

#[derive(Debug)]
pub struct Server {
    address: SocketAddr,
    database_url: String,
}

/************************************************************************************************/

impl Server {
    /*------------------------------------------------------------------------------------------*/

    pub fn new() -> Server {
        Server {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 119), // FIXME make address optional, don't assume 0.0.0.0 is correct
            database_url: DATA_BASE_URL.to_string(),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_binding_address(&mut self, address: SocketAddr) {
        self.address = address;
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_database_url(&mut self, dburl: String) {
        self.database_url = dburl
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn start(&self) -> Result<(), DialogueError> {
        let listener = TcpListener::bind(&self.address).or_else(|e| {
            Err(DialogueError::new(format!("{:?}", e))
                .add(sr(ErrorBindingListener, &[&self.address.to_string()])))
        })?;

        LogMessage::new(sr(
            LogListeningOn,
            &[&listener.local_addr().unwrap().to_string()],
        ))
        .show();

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    if self.accept_by_address(&addr) {
                        self.handle_connection(stream)
                    } else {
                        LogMessage::new(sr(LogRejectConnection, &[&addr.to_string()]))
                            .set_type(Error)
                            .show();
                    }
                }
                Err(e) => LogMessage::new(sr(LogCouldntGetClient, &[&e.to_string()]))
                    .set_type(Error)
                    .show(),
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn accept_by_address(&self, addr: &SocketAddr) -> bool {
        // TODO: implement black/white list functionality here
        // for now only connection from 127.0.0.1 are accepted

        addr.ip().to_string().eq("127.0.0.1")
    }

    /*------------------------------------------------------------------------------------------*/

    fn handle_connection(&self, stream: TcpStream) {
        // TODO: implement max connections!

        let dburl = self.database_url.clone();

        let _handler = spawn(move || {
            let mut session = Session::new(&stream, dburl);
            if let Err(e) = session.run() {
                e.show();
            }
        });
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
