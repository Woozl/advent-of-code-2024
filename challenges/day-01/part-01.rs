use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-01/input.txt"))?;

    let rows: Vec<Vec<isize>> = input
        .lines()
        .map(|pair| {
            pair.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    // transposed + sorted
    let columns: Vec<Vec<isize>> = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect::<Vec<isize>>())
        .map(|mut column| {
            column.sort();
            column
        })
        .collect();

    let distance = columns[0]
        .iter()
        .enumerate()
        .fold(0, |accum, (i, num)| accum + isize::abs(num - columns[1][i]));

    println!("Distance: {}", distance);

    Ok(())
}
