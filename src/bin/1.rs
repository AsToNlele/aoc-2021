#![allow(dead_code)]
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    // part_one().unwrap();
    part_two().unwrap();
}

fn part_one() -> std::io::Result<()> {
    let file = File::open("inputs/1.txt")?;
    let reader = BufReader::new(file);

    let mut increased = 0;
    let mut prev_value = 0;

    for line in reader.lines() {
        let value: i32 = match line?.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if prev_value == 0 {
            prev_value = value;
        }

        if value > prev_value {
            increased += 1;
        }
        prev_value = value;
    }

    println!("{}", increased);

    Ok(())
}

trait ToInt {
    fn to_int(&self) -> i32;
}

impl ToInt for String {
    fn to_int(&self) -> i32 {
        self.parse::<i32>().unwrap_or(0)
    }
}

fn part_two() -> std::io::Result<()> {
    let file = File::open("inputs/1.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut increased = 0;

    for i in 0..lines.len() {
        if i < 3 {
            continue;
        }
        let prev_sum = lines[i-3].to_int() + lines[i-2].to_int() + lines[i-1].to_int();
        let sum = lines[i-2].to_int() + lines[i-1].to_int() + lines[i].to_int();
        if sum > prev_sum {
           increased+=1; 
        }
    }
    println!("{}", increased);
    Ok(())
}
