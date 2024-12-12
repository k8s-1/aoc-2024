use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let path = "input.txt";

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        reports.push(report);
    }

    for report in &reports {

        for i in 0..report.len() - 1 { // Loop up to the second-to-last element
            let current = report[i];
            let next = report[i + 1];

            if current < next {
                println!("{} is less than {}", current, next);
            } else if current > next {
                println!("{} is greater than {}", current, next);
            } else {
                println!("{} is equal to {}", current, next);
            }
        }

    }

    Ok(())
}
