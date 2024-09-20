use std::io::{self, Write};

fn main() {
    print!("Type temperature: ");
    let mut temperature = String::new();

    io::stdout().flush();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Error parsing tempareture");

    print!("Type unit (possible values are c & f): ");
    let mut unit = String::new();
    io::stdout().flush();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read unit");
    let unit: char = unit.trim().parse().expect("Failed to parse unit");

    match unit {
        'c' => println!(
            "{temperature} celsius is {} fahrenheit",
            celsius_to_fahrenheit(temperature)
        ),
        'f' => println!(
            "{temperature} fahrenheit is {} celsius",
            fahrenheit_to_celsius(temperature)
        ),
        x => panic!("{x} is not a valid unit"),
    }

    println!("You typed {temperature}{unit}");
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (9.0 / 5.0 * temperature) + 32.0
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}
