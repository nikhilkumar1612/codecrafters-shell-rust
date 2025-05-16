use crate::builtins::commands::COMMANDS;

pub fn type_builtin_handler(input: String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 2 && COMMANDS.contains(&parts[1]) {
        println!("{} is a shell builtin", parts[1]);
    }
}