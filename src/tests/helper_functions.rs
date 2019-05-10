/************************************************************************************************/

use std::env;

/************************************************************************************************/

pub fn response_code(line: &str) -> i16 {
    let parts = split_line(line);
    if parts.is_empty() {
        0
    } else {
        match parts[0].parse::<i16>() {
            Ok(v) => v,
            Err(_) => -1,
        }
    }
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
