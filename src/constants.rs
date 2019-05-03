/************************************************************************************************/

pub mod cli {
    pub const ARG_ADDRESS_LONG: &str = "address";
    pub const ARG_ADDRESS_NAME: &str = "address";
    pub const ARG_ADDRESS_SHORT: &str = "a";
    pub const ARG_DATABASE_URL_LONG: &str = "database-url";
    pub const ARG_DATABASE_URL_NAME: &str = "dburl";
    pub const ARG_DATABASE_URL_SHORT: &str = "d";
    pub const ARG_VERBOSE_LONG: &str = "verbose";
    pub const ARG_VERBOSE_NAME: &str = "verbose";
    pub const ARG_VERBOSE_SHORT: &str = "v";
    pub const COMMAND_INSTALL_NAME: &str = "install";
    pub const COMMAND_START_NAME: &str = "start";
    pub const COMMAND_STOP_NAME: &str = "stop";
}

/************************************************************************************************/

pub mod default {
    pub const DATA_BASE_URL: &str = "postgresql://dialogue@localhost/dialog";
}

/************************************************************************************************/

pub mod response {
    use crate::response::Response;

    pub const SERVICE_AVAILABLE_POSTING_ALLOWED: &Response = &Response {
        code: 200,
        message: "Service available, posting allowed.",
    };
    pub const SERVICE_AVAILABLE_POSTING_PROHIBITED: &Response = &Response {
        code: 201,
        message: "Service available, posting prohibited.",
    };
}

/************************************************************************************************/
