use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let mut left_numbers = Vec::<i32>::new();
    let mut right_numbers = Vec::<i32>::new();

    let mut similarity = 0;

    let start = SystemTime::now();

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

    let mut score = 0;
    for (i, &number) in left_numbers.iter().enumerate() {
        if i > 0 && number == left_numbers[i - 1] {
            similarity += score;
        } else {
            score = check_similarity(number, right_numbers.clone());
            similarity += score;
        }
    }

    println!("{} {}", similarity, start.elapsed().unwrap().as_millis());
}

fn check_similarity(number: i32, mut vec: Vec<i32>) -> i32 {
    let count = vec.binary_search(&number).map_or(0, |idx| {
        let mut count = 1;
        let mut left = idx;
        let mut right = idx;

        while left > 0 && vec[left - 1] == number {
            count += 1;
            left -= 1;
        }

        while right + 1 < vec.len() && vec[right + 1] == number {
            count += 1;
            right += 1;
        }

        count
    });

    number * count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}