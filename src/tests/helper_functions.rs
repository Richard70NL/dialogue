/************************************************************************************************/

use std::env;

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
