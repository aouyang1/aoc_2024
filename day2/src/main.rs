use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{process::exit};


use crate::part1::report_safety::report_safety;
use crate::part2::problem_dampener::problem_dampener;

pub mod part1;
pub mod part2;

fn main() {
    let mut a: Vec<i32>;
    let mut safe: bool;
    let mut valid: i32 = 0;
    let mut valid_dampened: i32 = 0;
    if let Ok(lines) = read_lines("./src/data.txt") {
        for line in lines.flatten() {
            a = Vec::new();
            for value in line.split_whitespace() {
                let num: i32 = match value.parse() {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("Value {} is not a valid integer [{}]", value, e);
                        exit(1);
                    }
                };
                a.push(num)
            }

            safe = report_safety(a.clone());
            if safe {
                valid += 1;
            }
            safe = problem_dampener(a.clone());
            if safe {
                valid_dampened += 1;
            }
        }
    }
    println!("safe count: {}", valid);
    println!("dampened safe count: {}", valid_dampened);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


