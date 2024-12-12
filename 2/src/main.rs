use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let path = "input.txt";

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut list_of_lists: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        list_of_lists.push(numbers);
    }

    for list in &list_of_lists {

        for i in 0..numbers.len() - 1 { // Loop up to the second-to-last element
            let current = numbers[i];
            let next = numbers[i + 1];

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
