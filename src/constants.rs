/************************************************************************************************/

macro_rules! sconst {
    ($name:ident, $value:expr) => {
        pub const $name: &str = $value;
    };
}

/************************************************************************************************/

macro_rules! rconst {
    ($name:ident, $code:expr, $message:expr) => {
        pub const $name: &Response = &Response {
            code: $code,
            message: $message,
        };
    };
}

/************************************************************************************************/

pub mod cli {
    sconst!(ARG_ADDRESS_LONG, "address");
    sconst!(ARG_ADDRESS_NAME, "address");
    sconst!(ARG_ADDRESS_SHORT, "a");
    sconst!(ARG_DATABASE_URL_LONG, "database-url");
    sconst!(ARG_DATABASE_URL_NAME, "dburl");
    sconst!(ARG_DATABASE_URL_SHORT, "d");
    sconst!(ARG_TEST_DATA_LONG, "test-data");
    sconst!(ARG_TEST_DATA_NAME, "testdata");
    sconst!(ARG_TEST_DATA_SHORT, "t");
    sconst!(ARG_VERBOSE_LONG, "verbose");
    sconst!(ARG_VERBOSE_NAME, "verbose");
    sconst!(ARG_VERBOSE_SHORT, "v");
    sconst!(COMMAND_INSTALL_NAME, "install");
    sconst!(COMMAND_START_NAME, "start");
    sconst!(COMMAND_STOP_NAME, "stop");
}

/************************************************************************************************/

pub mod env {
    sconst!(CARGO_PKG_NAME, env!("CARGO_PKG_NAME"));
    sconst!(CARGO_PKG_VERSION, env!("CARGO_PKG_VERSION"));
}

/************************************************************************************************/

pub mod default {
    sconst!(DATA_BASE_URL, "postgresql://dialogue@localhost/dialogue");
}

/************************************************************************************************/

pub mod response {
    use crate::response::Response;

    rconst!(
        SERVICE_AVAILABLE_POSTING_ALLOWED,
        200,
        "Service available, posting allowed."
    );
    rconst!(
        SERVICE_AVAILABLE_POSTING_PROHIBITED,
        201,
        "Service available, posting prohibited."
    );
    rconst!(CONNECTION_CLOSING, 205, "Connection closing.");
    rconst!(UNKNOWN_COMMAND, 500, "Unknown command.");
    rconst!(INVALID_COMMAND, 501, "Invalid command or syntax error.");
    rconst!(CAPABILITIES_LIST_FOLLOWS, 101, "Capability list follows.");
    rconst!(HELP_TEXT_FOLLOWS, 100, "Help text follows.");
    rconst!(SERVER_DATE_TIME, 111, "{1}");
    rconst!(GROUP_SUCCESS, 211, "{1} {2} {3} {4}");
    rconst!(NO_SUCH_GROUP, 411, "No such newsgroup.");
    rconst!(INTERNAL_SERVER_ERROR, 400, "Internal server error.");
    rconst!(NO_GROUP_SELECTED, 412, "No group selected.");
    rconst!(
        LIST_GROUP_SUCCESS,
        211,
        "{1} {2} {3} {4} Article numbers follow."
    );
}

/************************************************************************************************/
