/************************************************************************************************/

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliVerboseHelp,
    CliStartAbout,
    CliStopAbout,
    CliInstallAbout,
    CliDatabaseUrlHelp,
    CliAddressHelp,
    /*------------------------------------------------------------------------------------------*/
    ErrorDialogueExit,
    ErrorWriteLongHelp,
    ErrorInvalidCommand,
    ErrorBindingListener,
    /*------------------------------------------------------------------------------------------*/
    LogConnectionAccepted,
    /*------------------------------------------------------------------------------------------*/
    ResponseServiceAvailPostAllowed,
    ResponseServiceAvailPostProhibited,
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
        Text::ErrorDialogueExit => "Dialogue exits with the following error(s):",
        Text::ErrorWriteLongHelp => "An error occured while writing the help information.",
        Text::ErrorInvalidCommand => "Invalid command.",
        Text::ErrorBindingListener => "Error while binding to address {1}.",
        /*--------------------------------------------------------------------------------------*/
        Text::LogConnectionAccepted => "Connection accepted.",
        /*--------------------------------------------------------------------------------------*/
        Text::ResponseServiceAvailPostAllowed => "Service available, posting allowed.",
        Text::ResponseServiceAvailPostProhibited => "Service available, posting prohibited.",
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
