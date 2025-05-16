mod builtins;

#[allow(unused_imports)]
use std::io::{self, Write};
use crate::builtins::echo::echo_builtin_handler;
use crate::builtins::type_builtin::type_builtin_handler;

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
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() > 0 {
                    if parts[0] == "echo" {
                        echo_builtin_handler(input_trimmed.to_string().clone())
                    }
                    else if parts[0] == "type" {
                        type_builtin_handler(input_trimmed.to_string().clone())
                    }
                    else {
                        println!("{}: command not found", input_trimmed)
                    }
                }
            }
        }
    }
}
