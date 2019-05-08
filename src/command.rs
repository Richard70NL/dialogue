/************************************************************************************************/

use crate::data::Range;
use crate::error::DialogueError;
use crate::types::*;

/************************************************************************************************/

#[derive(Debug)]
pub enum Command {
    Quit,
    Capabilities,
    Help,
    Date,
    Group(String),
    ListGroup(Option<String>, Option<Range>),
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
                "listgroup" => {
                    if command.len() > 1 {
                        let group_id = command[1].to_lowercase();

                        if command.len() > 2 {
                            match parse_range(&command[2]) {
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

fn parse_range(range_str: &str) -> Result<Range, DialogueError> {
    let v: Vec<&str> = range_str.split('-').collect();

    if v.is_empty() {
        Ok(Range { from: 0, to: 0 })
    } else if v.len() == 1 {
        let nr = parse_integer(v[0], 0)?;
        Ok(Range { from: nr, to: nr })
    } else {
        Ok(Range {
            from: parse_integer(v[0], 0)?,
            to: parse_integer(v[1], MAX_DB_INTEGER)?,
        })
    }
}

/************************************************************************************************/

fn parse_integer(s: &str, default: DbInteger) -> Result<DbInteger, DialogueError> {
    if s.is_empty() {
        Ok(default)
    } else {
        match s.parse::<DbInteger>() {
            Ok(i) => Ok(i),
            Err(e) => Err(DialogueError::new(format! {"{:?}", e})),
        }
    }
}

/************************************************************************************************/
