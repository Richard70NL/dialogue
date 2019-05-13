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

    test_101_capabilities_command(&mut client);
    test_102_help_command(&mut client);
    test_103_date_command();
    test_104_group_command();
    test_105_listgroup_command();

    test_950_unkown_command(&mut client);
    test_999_quit_connection(&mut client);

    // TODO: keep adding tests when adding more commands and server functionality
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
    let response = client.get_response(false);
    dbg!(&response);
    assert_eq!(response.get_code(), 201, "could not connect to the server");
}

/************************************************************************************************/

fn test_101_capabilities_command(client: &mut TestClient) {
    client.send_command("capabilities");
    let response = client.get_response(true);

    assert_eq!(
        response.get_code(),
        101,
        "invalid response code for CAPABILITIES command"
    );

    assert_eq!(response.get_body().len(), 3, "body should contain 3 lines");

    for (x, line) in response.get_body().iter().enumerate() {
        if x == 0 {
            assert!(
                line.starts_with("VERSION"),
                "first line should start with VERSION"
            )
        } else if x == 1 {
            assert!(
                line.starts_with("IMPLEMENTATION"),
                "second line should start with IMPLEMENTATION"
            )
        } else if x == 2 {
            assert!(
                line.starts_with("READER"),
                "third line should start with READER"
            )
        }
    }
}

/************************************************************************************************/

fn test_102_help_command(client: &mut TestClient) {
    client.send_command("help");

    let response = client.get_response(true);

    assert_eq!(
        response.get_code(),
        100,
        "invalid response code for HELP command"
    );

    assert!(
        !response.get_body().is_empty(),
        "invalid response body for HELP command"
    );
}

/************************************************************************************************/

fn test_103_date_command() {
    // TODO: implement date command tests
}

/************************************************************************************************/

fn test_104_group_command() {
    // TODO: implement group command tests
}

/************************************************************************************************/

fn test_105_listgroup_command() {
    // TODO: implement listgroup command tests
}

/************************************************************************************************/

fn test_950_unkown_command(client: &mut TestClient) {
    client.send_command("some unknown command");
    let response = client.get_response(false);

    assert_eq!(
        response.get_code(),
        500,
        "invalid response for an unknwon command"
    );
}

/************************************************************************************************/

fn test_999_quit_connection(client: &mut TestClient) {
    client.send_command("quit");
    let response = client.get_response(false);

    assert_eq!(
        response.get_code(),
        205,
        "could not quit savely from the server"
    );
}

/************************************************************************************************/
