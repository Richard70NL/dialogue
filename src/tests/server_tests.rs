/************************************************************************************************/

use crate::server::Server;
use crate::tests::helper_functions::response_code;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::str::FromStr;
use std::thread::spawn;
use std::time::Duration;

/************************************************************************************************/

const TEST_SERVER_ADDRESS: &str = "127.0.0.1:119";
const TEST_DATA_BASE_URL: &str = "postgresql://dialogue_test@localhost/dialogue_test";
const TEST_READ_TIMEOUT: u64 = 10;

/************************************************************************************************/

#[test]
fn full_server_test() {
    start_server_in_thread(); // this will non-block and is shutdown when testing is done
    match connect_to_server() {
        Ok(stream) => start_testing(stream),
        Err(e) => assert!(false, e),
    }
}

/************************************************************************************************/

fn start_server_in_thread() {
    let _handle = spawn(move || {
        let mut server = Server::new();
        server.set_binding_address(
            SocketAddr::from_str(TEST_SERVER_ADDRESS)
                .expect("could not set the binding address of the server"),
        );
        server.set_database_url(String::from(TEST_DATA_BASE_URL));
        match server.start() {
            Ok(()) => assert!(true, "server started correctly"),
            Err(e) => {
                e.show();
                assert!(false, "problem starting the server");
            }
        }
    });
}

/************************************************************************************************/

fn connect_to_server() -> Result<TcpStream, std::io::Error> {
    match TcpStream::connect(TEST_SERVER_ADDRESS) {
        Ok(stream) => {
            stream
                .set_read_timeout(Some(Duration::new(TEST_READ_TIMEOUT, 0)))
                .expect("could not set the read timeout");
            Ok(stream)
        }
        Err(e) => Err(e),
    }
}

/************************************************************************************************/

fn start_testing(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let _writer = BufWriter::new(&stream);
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("could not read the connection response");

    assert_eq!(response_code(&line), 201, "could not connect to the server");
    // TODO: add some more testing
}

/************************************************************************************************/
