/************************************************************************************************/

#[derive(Debug)]
pub enum Command {
    Quit,
    Capabilities,
    Help,
    Date,
    Group(String),
    Unknown(Vec<String>),
    Invalid(Vec<String>),
}

/************************************************************************************************/

impl Command {
    /*------------------------------------------------------------------------------------------*/

    pub fn parse(cmd: &str) -> Command {
        let command_line_string = String::from(cmd.trim());
        let iter = command_line_string.split_ascii_whitespace();
        let command: Vec<String> = iter.map(String::from).collect();

        if !command.is_empty() {
            match command[0].to_lowercase().as_str() {
                "quit" | "exit" | "logout" => Command::Quit,
                "capabilities" => Command::Capabilities,
                "help" => Command::Help,
                "date" => Command::Date,
                "group" => {
                    if command.len() > 1 {
                        Command::Group(command[1].to_lowercase())
                    } else {
                        Command::Invalid(command)
                    }
                }
                &_ => Command::Unknown(command),
            }
        } else {
            Command::Unknown(command)
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
