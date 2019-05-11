/************************************************************************************************/

use crate::tests::util::split_line;
use crate::tests::util::StringExtended;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;

/************************************************************************************************/

pub struct TestClient<'a> {
    _stream: &'a TcpStream,
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
}

/************************************************************************************************/

#[derive(Debug)]
pub struct TestResponse {
    parts: Vec<String>,
    body: Vec<String>,
}

/************************************************************************************************/

impl<'a> TestClient<'a> {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(stream: &'a TcpStream) -> TestClient {
        let reader = BufReader::new(stream);
        let writer = BufWriter::new(stream);

        TestClient {
            _stream: stream,
            reader,
            writer,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn get_response(&mut self, multi_line: bool) -> TestResponse {
        let mut line = String::new();
        let mut buffer = String::new();

        loop {
            line.clear();
            self.reader
                .read_line(&mut line)
                .expect("could not read the response");
            buffer.push_str(&line);

            if !multi_line {
                break;
            } else if buffer.last_line() == "." {
                buffer = buffer.remove_last_line();
                break;
            }
        }

        TestResponse::parse(buffer)
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn send_command(&mut self, command_str: &str) {
        let mut command = String::from(command_str);
        command.push_str("\n");

        self.writer
            .write(command.as_bytes())
            .expect("could not send quit command");
        self.writer.flush().expect("could not flush the writer");
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

impl TestResponse {
    /*------------------------------------------------------------------------------------------*/

    fn parse(string_buffer: String) -> TestResponse {
        if string_buffer.is_empty() {
            TestResponse {
                parts: Vec::new(),
                body: Vec::new(),
            }
        } else {
            let lines: Vec<String> = string_buffer.lines().map(String::from).collect();
            let mut parts: Vec<String> = Vec::new();
            let mut body: Vec<String> = Vec::new();

            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    parts = split_line(line);
                } else {
                    body.push(line.to_string());
                }
            }

            TestResponse { parts, body: body }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn get_code(&self) -> i16 {
        if self.parts.is_empty() {
            0
        } else {
            match self.parts[0].parse::<i16>() {
                Ok(v) => v,
                Err(_) => -1,
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn get_body(&self) -> &Vec<String> {
        &self.body
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
