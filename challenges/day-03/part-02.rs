use regex::Regex;
use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let re = Regex::new(
        r"(?<do>do(?<dont>n't)?\(\)|mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\))",
    )?;

    let mut mul_enabled = true;
    let sum_of_products = re.captures_iter(&input).fold(0, |accum, caps| {
        let has_dont_cap = caps.name("dont").is_some();
        let first_cap = caps.name("first").map(|s| s.as_str());
        let second_cap = caps.name("second").map(|s| s.as_str());

        if let (Some(first), Some(second)) = (first_cap, second_cap) {
            return if mul_enabled {
                accum + (first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap())
            } else {
                accum
            };
        } else if has_dont_cap {
            mul_enabled = false;
        } else {
            mul_enabled = true;
        }

        accum
    });

    println!("Sum of products: {sum_of_products}");

    Ok(())
}
