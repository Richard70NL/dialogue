/************************************************************************************************/

use crate::tests::helper_functions::split_line;
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

pub struct TestResponse {
    parts: Vec<String>,
    _body: Vec<String>,
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

    pub fn get_single_line_response(&mut self) -> TestResponse {
        let mut line = String::new();

        self.reader
            .read_line(&mut line)
            .expect("could not read the connection response");

        TestResponse::parse(line)
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
                _body: Vec::new(),
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

            TestResponse { parts, _body: body }
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
}

/************************************************************************************************/
