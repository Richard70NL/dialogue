/************************************************************************************************/

use crate::server::Server;
use crate::tests::test_client::TestClient;
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

    // create a simple reader/writer client and connect to the above server
    let stream =
        TcpStream::connect(TEST_CONNECTION_ADDRESS).expect("could not connect to the server");

    stream
        .set_read_timeout(Some(Duration::new(TEST_READ_TIMEOUT, 0)))
        .expect("could not set the read timeout");

    let mut client = TestClient::new(&stream);

    // perform all tests
    test_001_initial_connection(&mut client);
    test_999_quit_connection(&mut client);

    // TODO: add some more testing
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

fn test_001_initial_connection(client: &mut TestClient) {
    let response = client.get_single_line_response();
    assert_eq!(response.get_code(), 201, "could not connect to the server");
}

/************************************************************************************************/

fn test_999_quit_connection(client: &mut TestClient) {
    client.send_command("quit");
    let response = client.get_single_line_response();

    assert_eq!(
        response.get_code(),
        205,
        "could not quit savely from the server"
    );
}

/************************************************************************************************/
