use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{process::exit};

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

    /*
    let mut a = [3, 4, 2, 1, 3, 3];
    let mut b = [4, 3, 5, 3, 9, 3];
    */
    assert_eq!(a_id.len(), b_id.len());

    a_id.sort();
    b_id.sort();

    let n = a_id.len();

    let mut total = 0;
    for i in 0..n {
        let diff: i32 = a_id[i] - b_id[i];
        total = total + diff.abs();
        // println!("a: {} b: {} diff: {}, total: {}", a[i], b[i], diff.abs(), total);
    }
    println!("total diff: {total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
