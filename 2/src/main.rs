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
        println!("{:?}", list);
    }

    Ok(())
}
