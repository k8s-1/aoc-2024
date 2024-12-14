use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut list_of_lists: Vec<Vec<i32>> = Vec::new();

    for l in reader.lines() {
        let l = l?;
        let numbers: Vec<i32> = l
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        list_of_lists.push(numbers);
    }

    println!("Total safe items: {}", total_safe(&mut list_of_lists));
    println!("Total dampened safe items: {}", total_damp_safe(&mut list_of_lists));

    Ok(())
}

fn is_decreasing(list: &[i32]) -> bool {
    for w in list.windows(2) {
        if w[0] > w[1] {
            return true;
        }
    }
    false
}

fn is_safe(list: &mut [i32]) -> bool {
    if is_decreasing(list) {
        list.reverse();
    }

    for w in list.windows(2) {
        let diff = w[1] - w[0];
        match diff {
            d if d > 0 && d < 4 => continue,
            _ => return false,
        }
    }

    true
}

fn remove_one<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let mut temp = input.clone();
        temp.remove(i);
        result.push(temp);
    }
    result
}

fn is_damp_safe(list: &mut [i32]) -> bool {
    if is_safe(list) {
        return true
    }

    let sublists = remove_one(list.to_vec());

    for mut l in sublists {
        let x: &mut [i32] = l.as_mut_slice();
        if is_safe(x) {
            return true
        }
    }

    false
}

fn total_safe(list_of_lists: &mut Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for li in list_of_lists {
        if is_safe(li) {
            sum += 1;
        }
    }
    sum
}

fn total_damp_safe(list_of_lists: &mut Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for li in list_of_lists {
        if is_damp_safe(li) {
            sum += 1;
        }
    }
    sum
}
