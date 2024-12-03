use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-02/input.txt"))?;

    let reports: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let num_safe_reports = reports.iter().fold(0, |accum, level| {
        let deltas = level
            .windows(2)
            .map(|window| window[1] - window[0]);

        let mut is_descending: Option<bool> = None;
        for delta in deltas {
            if delta.abs() < 1 || delta.abs() > 3 { 
                return accum 
            }

            if let Some(is_descending) = is_descending {
                if is_descending && delta > 0 || !is_descending && delta < 0 {
                    return accum
                }
            }
            else {
                is_descending = Some(delta < 0)
            }
        }

        accum + 1
    });

    println!("Number of safe levels: {}", num_safe_reports);

    Ok(())
}
