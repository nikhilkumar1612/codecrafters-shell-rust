#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();
        let parts: Vec<&str> = input_trimmed.split_whitespace().collect();

        match input_trimmed {
            "exit 0" => break,
            str => {
                let parts: Vec<&str> = str.split_whitespace().collect();
                if parts.len() > 1 && parts[0] == "echo" {
                    for part in &parts[1..] {
                        print!("{} ", part);
                    }
                    println!();
                } else {
                    println!("{}: command not found", input_trimmed)
                }
            }
        }
    }
}
