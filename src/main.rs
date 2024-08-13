use std::{
    io::{
	self,
	Write},
    collections::HashMap
};

fn main()
{
    println!("Enter a vector of integers:");
    Write::flush(&mut std::io::stdout()).unwrap();
    let vec = read_i32_vector();
    println!("median is: {}\nmode is: {}", get_median(&vec), get_mode(&vec));
}

fn get_median(vec: &Vec<i32>) -> f64
{
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();
    match sorted_vec.len() {
	i if i % 2 == 0 => (sorted_vec[i / 2] + sorted_vec[i / 2 - 1]) as f64 / 2.0,
	i => sorted_vec[i / 2] as f64
    }
}

fn get_mode(vec: &Vec<i32>) -> i32
{
    let mut occurences = HashMap::new();
    for i in vec
    {
	let occurence = occurences.entry(i).or_insert(0);
	*occurence += 1;
    }
    **occurences
	.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
	.unwrap()
}

fn read_line() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    input.trim().to_string()
}

fn read_i32_vector() -> Vec<i32>
{
    read_line().split_whitespace().map(|s| s.trim().parse().expect("Not i32")).collect()
}
