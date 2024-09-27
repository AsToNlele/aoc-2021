use aoc_2021::get_lines;
use aoc_2021::ToInt;

fn main() -> std::io::Result<()> {
    let lines = get_lines("inputs/1.txt")?;
    part_one(&lines).unwrap();
    part_two(&lines).unwrap();
    Ok(())
}

fn part_one(lines: &Vec<String>) -> std::io::Result<()> {
    let mut increased = 0;
    let mut prev_value = 0;

    for line in lines {
        let value: i32 = match line.parse() {
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
    println!("Part 1: {}", increased);
    Ok(())
}

fn part_two(lines: &[String]) -> std::io::Result<()> {
    let mut increased = 0;

    for i in 0..lines.len() {
        if i < 3 {
            continue;
        }
        let prev_sum = lines[i - 3].to_int() + lines[i - 2].to_int() + lines[i - 1].to_int();
        let sum = lines[i - 2].to_int() + lines[i - 1].to_int() + lines[i].to_int();
        if sum > prev_sum {
            increased += 1;
        }
    }
    println!("Part 2: {}", increased);
    Ok(())
}
