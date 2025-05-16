pub fn echo_builtin_handler(input: String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 1 {
        for part in &parts[1..] {
            print!("{} ", part);
        }
    }
    println!();
}