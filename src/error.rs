/************************************************************************************************/

use crate::text::s;
use crate::text::Text::*;

/************************************************************************************************/

#[derive(Debug)]
pub struct DialogueError {
    messages: Vec<String>,
    error_type: DialogueErrorType,
}

/************************************************************************************************/

#[derive(Debug)]
pub enum DialogueErrorType {
    Generic,
    NoSuchGroup,
}

/************************************************************************************************/

impl DialogueError {
    /*------------------------------------------------------------------------------------------*/

    pub fn new(message: String) -> DialogueError {
        DialogueError {
            messages: Vec::new(),
            error_type: DialogueErrorType::Generic,
        }
        .add(message)
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn add(mut self, message: String) -> DialogueError {
        self.messages.insert(0, message);
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn set_type(mut self, error_type: DialogueErrorType) -> DialogueError {
        self.error_type = error_type;
        self
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn get_type(&self) -> &DialogueErrorType {
        &self.error_type
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn show(&self) {
        // FIXME: use error logging (LogMessage)

        eprintln!("{}", s(ErrorDialogue));
        for msg in &self.messages {
            eprintln!("- {}", msg);
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
