use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;

    let regex = Regex::new(r"(?m)mul\(\d{1,3},\d{1,3}\)").unwrap();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let string = line.unwrap();
            let result = regex.captures_iter(string.as_str());

            for mat in result {
                sum += calc_op(mat.get(0).unwrap().as_str().replace("mul(", "").to_string());
            }
        }
    }

    println!("{}", sum);
}

fn calc_op(op: String) -> i32 {
    let vec = op.split(",").collect::<Vec<&str>>();
    vec[0].parse::<i32>().unwrap() * vec[1][0..vec[1].len() - 1].parse::<i32>().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}