/************************************************************************************************/

use crate::data::GroupId;
use crate::data::Range;

/************************************************************************************************/

#[derive(Debug)]
pub enum Command {
    Quit,
    Capabilities,
    Help,
    Date,
    Group(GroupId),
    ListGroup(Option<GroupId>, Option<Range>),
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
                        Command::Group(GroupId::from(&command[1]))
                    } else {
                        Command::Invalid(command)
                    }
                }
                "listgroup" => {
                    if command.len() > 1 {
                        let group_id = GroupId::from(&command[1]);

                        if command.len() > 2 {
                            match Range::parse(&command[2]) {
                                Ok(range) => Command::ListGroup(Some(group_id), Some(range)),
                                Err(_) => Command::Invalid(command),
                            }
                        } else {
                            Command::ListGroup(Some(group_id), None)
                        }
                    } else {
                        Command::ListGroup(None, None)
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
