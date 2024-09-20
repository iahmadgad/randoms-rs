use std::io::{self, Write};

fn main() {
    let n1: i32 = read_i32("First Number: ");
    let n2: i32 = read_i32("Second Number: ");
    let operation = read_i32("1: +\n2: -\n3: *\n4: /\nOperation: ");
    match operation {
        1 => println!("Result is: {}", n1 + n2),
        2 => println!("Result is: {}", n1 - n2),
        3 => println!("Result is: {}", n1 * n2),
        4 => println!("Result is: {}", n1 / n2),
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => panic!("Invalid operation"),
    }
}

/// # Performance
/// Allocates a new `String` every time it is called.
///
/// # Panic
/// Panics if it fails to read from `Stdin`
fn read(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // Ensure the message is printed immediately
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret.trim().to_string() // Remove the newline character and return the string
}

fn read_i32(message: &str) -> i32 {
    let input = read(message);
    match input.trim().parse::<i32>() {
        Ok(num) => return num,
        Err(_) => panic!("Not i32"),
    }
}
