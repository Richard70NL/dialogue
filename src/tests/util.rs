/************************************************************************************************/

use std::env;

/************************************************************************************************/

pub trait StringExtended {
    fn last_line(&self) -> &str;
    fn remove_last_line(self) -> String;
}

/************************************************************************************************/

impl StringExtended for String {
    /*------------------------------------------------------------------------------------------*/

    fn last_line(&self) -> &str {
        self.lines()
            .last()
            .expect("should have returned the last line")
    }

    /*------------------------------------------------------------------------------------------*/

    fn remove_last_line(self) -> String {
        let mut lines: Vec<&str> = self.lines().collect();

        let mut buffer = String::new();

        if !lines.is_empty() {
            lines.remove(lines.len() - 1);
            lines.iter().for_each(|s| {
                if !buffer.is_empty() {
                    buffer.push('\n')
                };
                buffer.push_str(s);
            })
        }

        buffer
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

pub fn split_line(val: &str) -> Vec<String> {
    let line = String::from(val);
    let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
    parts
}

/************************************************************************************************/

pub fn _get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("couldn't interpret {}: {}", key, e);
            String::new()
        }
    }
}

/************************************************************************************************/
