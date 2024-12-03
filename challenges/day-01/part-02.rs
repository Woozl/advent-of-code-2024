use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-01/input.txt"))?;

    println!("Hello from day 01 part 2");
    println!("Input file: {}", &input);

    Ok(())
}
