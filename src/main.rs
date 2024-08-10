use std::io::{
    self,
    Write
};

const VOWELS: [char; 5] = ['a', 'e', 'u', 'i', 'o'];

fn main()
{
    print!("Enter string: ");
    Write::flush(&mut std::io::stdout()).unwrap();
    let string = read_line();
    println!("\nIgpay Atinlay: {}", convert_string(&string));
}

fn convert_string(s : &str) -> String
{
    s.split_whitespace().map(|s| convert_word(&s)).collect()
}

fn convert_word(s: &str) -> String
{
    let (first_char, rest) = (s.chars().nth(0).unwrap(), s.get(1..).unwrap());
    if VOWELS.contains(&first_char)
    {
	format!("{first_char}{rest}hay ")
    }
    else
    {
	format!("{rest}{first_char}ay ")
    }
}

fn read_line() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    input.trim().to_string()
}
