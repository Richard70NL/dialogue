/************************************************************************************************/

use chrono::prelude::*;
use std::net::SocketAddr;

/************************************************************************************************/

pub struct LogMessage {
    message_type: LogMessageType,
    message: String,
    response_code: Option<u16>,
    client_addr: Option<SocketAddr>,
}

/************************************************************************************************/

pub enum LogMessageType {
    Log,
    Error,
}

/************************************************************************************************/

impl LogMessage {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(message: String) -> LogMessage {
        LogMessage {
            message_type: LogMessageType::Log,
            message,
            response_code: None,
            client_addr: None,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_type(mut self, message_type: LogMessageType) -> LogMessage {
        self.message_type = message_type;
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_response_code(mut self, response_code: u16) -> LogMessage {
        self.response_code = Some(response_code);
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_client_addr(mut self, client_addr: SocketAddr) -> LogMessage {
        self.client_addr = Some(client_addr);
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show(&self) {
        let mut line = String::new();
        let separator = " - ";
        let dt = Local::now();

        line.push_str(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string().as_str());
        line.push_str(separator);
        if let Some(client_addr) = &self.client_addr {
            line.push_str(&client_addr.to_string());
        }
        line.push_str(separator);
        if let Some(response_code) = &self.response_code {
            line.push_str(&response_code.to_string());
        }
        line.push_str(separator);
        line.push_str(self.message.as_str());

        #[cfg(not(test))] // don't log while testing
        match &self.message_type {
            LogMessageType::Log => println!("L{}{}", separator, line),
            LogMessageType::Error => eprintln!("E{}{}", separator, line),
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
