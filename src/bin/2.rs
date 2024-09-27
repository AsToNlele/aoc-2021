use aoc_2021::{get_lines, ToInt};
use std::io::Result;

fn main() -> Result<()> {
    let lines = get_lines("inputs/2.txt")?;
    part_one(&lines);
    part_two(&lines);
    Ok(())
}

fn part_one(lines: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let direction = split[0];
        let value = split[1].to_int();

        match direction {
            "down" => {
                depth += value;
            }
            "up" => {
                depth -= value;
            }
            _ => {
                horizontal += value;
            }
        };
    }

    println!("Part 1: {}", horizontal * depth);
}

fn part_two(lines: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let direction = split[0];
        let value = split[1].to_int();

        match direction {
            "down" => {
                aim += value;
            }
            "up" => {
                aim -= value;
            }
            _ => {
                horizontal += value;
                depth += aim * value;
            }
        };
    }

    println!("Part 2: {}", horizontal * depth);
}
