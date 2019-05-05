/************************************************************************************************/
// TODO: Reconsider this method of producing text. Maybe constants is better.

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliVerboseHelp,
    CliStartAbout,
    CliStopAbout,
    CliInstallAbout,
    CliDatabaseUrlHelp,
    CliAddressHelp,
    /*------------------------------------------------------------------------------------------*/
    ErrorDialogue,
    ErrorWriteLongHelp,
    ErrorInvalidCommand,
    ErrorBindingListener,
    ErrorWhileWriting,
    ErrorConnectingDb,
    ErrorInstallingSchema,
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
        /*--------------------------------------------------------------------------------------*/
        Text::ErrorDialogue => "There was an problem because of the following error(s):",
        Text::ErrorWriteLongHelp => "An error occured while writing the help information.",
        Text::ErrorInvalidCommand => "Invalid command.",
        Text::ErrorBindingListener => "Error while binding to address {1}.",
        Text::ErrorWhileWriting => "There was an error while writing to the stream.",
        Text::ErrorConnectingDb => "Error while connecting to the database.",
        Text::ErrorInstallingSchema => "Error while installing or upgrading the database schema.",
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
        /*--------------------------------------------------------------------------------------*/
    }
}

/************************************************************************************************/

pub fn so(text: Text) -> String {
    s(text).to_string()
}

/************************************************************************************************/

pub fn sr(text: Text, values: &[&str]) -> String {
    let mut msg = String::from(s(text));

    for (i, v) in values.iter().enumerate() {
        let mut place_holder = String::new();
        place_holder.push('{');
        place_holder.push_str(&(i + 1).to_string());
        place_holder.push('}');
        msg = msg.replace(&place_holder, v);
    }

    msg
}

/************************************************************************************************/
