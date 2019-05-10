/************************************************************************************************/

use crate::server::Server;
use crate::tests::helper_functions::response_code;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
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
    // this will non-block and is shutdown when testing is done
    start_server_in_thread(
        SocketAddr::from_str(TEST_BINDING_ADDRESS).expect("could not create a socket address"),
    );

    match connect_to_server() {
        Ok(stream) => start_testing(stream),
        Err(e) => assert!(false, e),
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
    let mut writer = BufWriter::new(&stream);

    test_001_initial_connection(&mut reader);
    test_999_quit_connection(&mut reader, &mut writer);

    // TODO: add some more testing
}

/************************************************************************************************/

fn test_001_initial_connection(reader: &mut BufReader<&TcpStream>) {
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("could not read the connection response");

    assert_eq!(response_code(&line), 201, "could not connect to the server");
}

/************************************************************************************************/

fn test_999_quit_connection(
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
) {
    let mut line = String::new();

    writer
        .write(b"quit\n")
        .expect("could not send quit command");
    writer.flush().expect("could not flush the writer");

    reader
        .read_line(&mut line)
        .expect("could not read the quit response");

    assert_eq!(
        response_code(&line),
        205,
        "could not quit savely from the server"
    );
}

/************************************************************************************************/
