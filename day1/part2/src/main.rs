use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{process::exit};
use std::collections::HashMap;

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

    // let mut a = vec![3, 4, 2, 1, 3, 3];
    // let mut b = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(a_id.len(), b_id.len());

    let n = a_id.len();

    let mut b_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        match b_map.get(&b_id[i]) {
            Some(num) => {
                b_map.insert(b_id[i], num+1);
            },
            None => {
                b_map.insert(b_id[i], 1);
            }
        }
    }

    // for (k, v) in &b_map {
    //     println!("{k}: {v}");
    // }

    let mut score: i32 = 0;
    for i in 0..n {
        match b_map.get(&a_id[i]) {
            Some(num) => {
                score = score + a_id[i] * num;
            },
            None => (),
        }
    }
    println!("{}", score)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
