use std::io::{self, Write};

pub fn mreadln(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    print!("\x1b[0m");
    input.trim().to_string()
}