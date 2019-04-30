/************************************************************************************************/

use crate::constants::*;
use crate::error::DialogueError;
use crate::text::sr;
use crate::text::Text::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;

/************************************************************************************************/

#[derive(Debug)]
pub struct Server {
    address: IpAddr,
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

        loop {
            match listener.accept() {
                Ok((_socket, addr)) => println!("new client: {:?}", addr),
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
