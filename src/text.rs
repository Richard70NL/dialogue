/************************************************************************************************/

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliVerboseHelp,
    CliListenAbout,
    /*------------------------------------------------------------------------------------------*/
    ErrorDialogueExit,
    ErrorWriteLongHelp,
    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

pub fn s(text: Text) -> &'static str {
    match text {
        /*--------------------------------------------------------------------------------------*/
        Text::CliVerboseHelp => "Use verbose output.",
        Text::CliListenAbout => "Start the server and listen to incomming connections.",
        /*--------------------------------------------------------------------------------------*/
        Text::ErrorDialogueExit => "Dialogue exits with the following error(s):",
        Text::ErrorWriteLongHelp => "An error occured while writing the help information.",
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
