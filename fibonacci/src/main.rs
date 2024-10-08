use std::io::{self, Write};

fn main() {
    print!("Please type number n: ");

    let mut n = String::new();

    io::stdout().flush();
    io::stdin().read_line(&mut n).expect("Error reading n");

    let n: u32 = n.trim().parse().expect("Error parsing n");

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
