/************************************************************************************************/

#[derive(Debug)]
pub enum Command {
    Quit,
    Unknown(Vec<String>),
}

/************************************************************************************************/

impl Command {
    /*------------------------------------------------------------------------------------------*/

    pub fn parse(cmd: &String) -> Command {
        let command_line_string = String::from(cmd.trim());
        let iter = command_line_string.split_ascii_whitespace();
        let command: Vec<String> = iter.map(|s| String::from(s)).collect();

        if command.len() > 0 {
            match command[0].to_lowercase().as_str() {
                "quit" | "exit" | "logout" => Command::Quit,
                &_ => Command::Unknown(command),
            }
        } else {
            Command::Unknown(command)
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
