use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{6}\+\d{4}").expect("Failed to compile regex");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if re.is_match(&line) {
            if !buffer.is_empty() {
                println!("{}", buffer);
                buffer.clear();
            }
        }
        buffer.push_str(&format!(" {}", line));
    }

    if !buffer.is_empty() {
        println!("{}", buffer);
    }
}
