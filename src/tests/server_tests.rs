/************************************************************************************************/

use crate::server::Server;
use crate::tests::helper_functions::get_env_var;
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

const TEST_BINDING_ADDRESS: &str = "0.0.0.0:119";
const TEST_CONNECTION_ADDRESS: &str = "127.0.0.1:119";
const TEST_DATA_BASE_URL: &str = "postgresql://dialogue_test@localhost/dialogue_test";
const TEST_READ_TIMEOUT: u64 = 10;

/************************************************************************************************/

#[test]
fn full_server_test() {
    // FIXME: on travis the full_server_test is not allowed to open a listener on :119 for all IPs
    // skip full_server_test on travis/linux
    if get_env_var("TRAVIS_OS_NAME") != "linux" {
        // this will non-block and is shutdown when testing is done
        start_server_in_thread(
            SocketAddr::from_str(TEST_BINDING_ADDRESS).expect("could not create a socket address"),
        );

        match connect_to_server() {
            Ok(stream) => start_testing(stream),
            Err(e) => assert!(false, e),
        }
    }
}

/************************************************************************************************/

fn start_server_in_thread(address: SocketAddr) {
    let _handle = spawn(move || {
        let mut server = Server::new();

        server.set_binding_address(address);
        server.set_database_url(String::from(TEST_DATA_BASE_URL));

        match server.start() {
            Ok(()) => assert!(true, "server started correctly"),
            Err(e) => {
                e.show();
                assert!(false, "problem starting the server on address {}", address);
            }
        }
    });
}

/************************************************************************************************/

fn connect_to_server() -> Result<TcpStream, std::io::Error> {
    match TcpStream::connect(TEST_CONNECTION_ADDRESS) {
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
