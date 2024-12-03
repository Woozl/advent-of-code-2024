use std::{collections::HashMap, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-01/input.txt"))?;

    let rows: Vec<Vec<usize>> = input
        .lines()
        .map(|pair| {
            pair.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // transposed
    let columns: Vec<Vec<usize>> = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect::<Vec<usize>>())
        .collect();

    let mut freq_map = HashMap::new();
    columns[1]
        .iter()
        .for_each(|num| *freq_map.entry(num).or_insert(0) += 1);

    let similarity_score = columns[0].iter().fold(0, |accum, num| {
        accum + (num * freq_map.get(num).unwrap_or(&0))
    });

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
