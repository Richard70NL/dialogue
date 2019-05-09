/************************************************************************************************/

use crate::error::DialogueError;
use crate::types::DbInteger;

/************************************************************************************************/

pub fn string_format(text: String, values: &[&str]) -> String {
    let mut msg = text;

    if !values.is_empty() {
        for (i, v) in values.iter().enumerate() {
            let mut place_holder = String::new();
            place_holder.push('{');
            place_holder.push_str(&(i + 1).to_string());
            place_holder.push('}');
            msg = msg.replace(&place_holder, v);
        }
    }

    msg
}

/************************************************************************************************/

pub fn str_format(text: &str, values: &[&str]) -> String {
    string_format(String::from(text), values)
}

/************************************************************************************************/

pub fn parse_integer(s: &str, default: DbInteger) -> Result<DbInteger, DialogueError> {
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
