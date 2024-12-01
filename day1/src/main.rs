use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{process::exit};

use crate::part1::difference::difference;
use crate::part2::similarity::similarity;

pub mod part1;
pub mod part2;

fn main() {
    let mut a_id: Vec<i32> = Vec::new();
    let mut b_id: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./src/data.txt") {
        for line in lines.flatten() {
            for (i, value) in line.split_whitespace().enumerate() {
                let num: i32 = match value.parse() {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("Value {} is not a valid integer [{}]", value, e);
                        exit(1);
                    }
                };
                match i {
                    0 => a_id.push(num),
                    1 => b_id.push(num),
                    _ => {
                        eprintln!("expected at most 2 elements");
                        exit(1);
                    }
                }
            }
        }
    }

    println!("total diff: {}", difference(a_id.clone(), b_id.clone()));
    println!("similarity: {}", similarity(a_id.clone(), b_id.clone()));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
