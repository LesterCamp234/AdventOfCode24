use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_numbers = Vec::<i32>::new();
    let mut right_numbers = Vec::<i32>::new();

    let mut distance = 0;

    if let Ok(lines) = read_lines("./id.txt") {
        for line in lines {
            let string = line.unwrap();
            let numbers: Vec<_> = string.split(' ').collect();

            left_numbers.push(numbers[0].parse::<i32>().unwrap());
            right_numbers.push(numbers[3].parse::<i32>().unwrap());
        }
    }
    left_numbers.sort();
    right_numbers.sort();

    for i in 0..left_numbers.len() {
        distance = distance + (left_numbers[i] - right_numbers[i]).abs();
    }

    println!("{}", distance);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}