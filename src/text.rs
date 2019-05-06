/************************************************************************************************/
// TODO: Reconsider this method of producing text. Maybe constants is better.

/************************************************************************************************/

use crate::util::str_format;

/************************************************************************************************/

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliVerboseHelp,
    CliStartAbout,
    CliStopAbout,
    CliInstallAbout,
    CliDatabaseUrlHelp,
    CliAddressHelp,
    CliTestDataHelp,
    /*------------------------------------------------------------------------------------------*/
    ErrorDialogue,
    ErrorWriteLongHelp,
    ErrorInvalidCommand,
    ErrorBindingListener,
    ErrorWhileWriting,
    ErrorConnectingDb,
    ErrorInstallingSchema,
    ErrorReadingLine,
    /*------------------------------------------------------------------------------------------*/
    LogConnectionAccepted,
    LogRejectConnection,
    LogCouldntGetClient,
    LogInitializeServer,
    LogStartServer,
    LogDone,
    LogListeningOn,
    LogConnectingToDb,
    LogInstallingDbSchema,
    LogInstallingTestData,
    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

pub fn s(text: Text) -> &'static str {
    match text {
        /*--------------------------------------------------------------------------------------*/
        Text::CliVerboseHelp => "Use verbose output.",
        Text::CliStartAbout => "Start the server and listen for incomming connections.",
        Text::CliStopAbout => "Stop a running server.",
        Text::CliInstallAbout => "Installs or upgrades the database schema.",
        Text::CliDatabaseUrlHelp => "Database connect string.",
        Text::CliAddressHelp => "Binding IP address.",
        Text::CliTestDataHelp => "Install test data.",
        /*--------------------------------------------------------------------------------------*/
        Text::ErrorDialogue => "There was an problem because of the following error(s):",
        Text::ErrorWriteLongHelp => "An error occured while writing the help information.",
        Text::ErrorInvalidCommand => "Invalid command.",
        Text::ErrorBindingListener => "Error while binding to address {1}.",
        Text::ErrorWhileWriting => "There was an error while writing to the stream.",
        Text::ErrorConnectingDb => "Error while connecting to the database.",
        Text::ErrorInstallingSchema => "Error while installing or upgrading the database schema.",
        Text::ErrorReadingLine => "Error while reading a line from the stream.",
        /*--------------------------------------------------------------------------------------*/
        Text::LogConnectionAccepted => "Connection accepted.",
        Text::LogRejectConnection => "rejected connection from {1}.",
        Text::LogCouldntGetClient => "Couldn't get client due to: {1}.",
        Text::LogInitializeServer => "Initializing the server.",
        Text::LogStartServer => "Start the server.",
        Text::LogDone => "Done!",
        Text::LogListeningOn => "Listening on {1}.",
        Text::LogConnectingToDb => "Connecting to the database.",
        Text::LogInstallingDbSchema => "Installing or upgrading database schema.",
        Text::LogInstallingTestData => "Installing test data.",
        /*--------------------------------------------------------------------------------------*/
    }
}

/************************************************************************************************/

pub fn so(text: Text) -> String {
    s(text).to_string()
}

/************************************************************************************************/

pub fn sr(text: Text, values: &[&str]) -> String {
    str_format(s(text), values)
}

/************************************************************************************************/
