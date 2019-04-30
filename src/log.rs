/************************************************************************************************/

use chrono::prelude::*;

/************************************************************************************************/

pub struct LogMessage {
    message_type: LogMessageType,
    message: String,
    response_code: u16,
}

/************************************************************************************************/

pub enum LogMessageType {
    Log,
    Error,
}

/************************************************************************************************/

impl LogMessage {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(response_code: u16, message: String) -> LogMessage {
        LogMessage {
            message_type: LogMessageType::Log,
            message: message,
            response_code: response_code,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_type(mut self, message_type: LogMessageType) -> LogMessage {
        self.message_type = message_type;
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show(&self) {
        let mut s = String::new();
        let separator = " - ";
        let process_id = std::process::id().to_string();
        let mut thread_id = format!("{:?}", std::thread::current().id());
        let dt = Local::now();

        thread_id = thread_id.trim_end_matches(')').to_string();
        thread_id = thread_id.trim_start_matches("ThreadId(").to_string();

        s.push_str(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string().as_str());
        s.push_str(separator);
        s.push_str(&process_id);
        s.push_str(separator);
        s.push_str(&thread_id);
        s.push_str(separator);
        s.push_str(&self.response_code.to_string());
        s.push_str(separator);
        s.push_str(self.message.as_str());

        match &self.message_type {
            LogMessageType::Log => println!("L{}{}", separator, s),
            LogMessageType::Error => eprintln!("E{}{}", separator, s),
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
