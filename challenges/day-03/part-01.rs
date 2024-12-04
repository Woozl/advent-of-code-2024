use regex::Regex;
use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)")?;

    let sum_of_products = re.captures_iter(&input).fold(0, |accum, caps| {
        let first = caps.name("first").unwrap().as_str();
        let second = caps.name("second").unwrap().as_str();
        accum + (first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap())
    });

    println!("Sum of products: {sum_of_products}");

    Ok(())
}
