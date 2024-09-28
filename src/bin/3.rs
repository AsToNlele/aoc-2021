use std::io::Result;

use aoc_2021::get_lines;

fn main() -> Result<()> {
    let lines = get_lines("inputs/3.txt")?;
    part_one(&lines);

    Ok(())
}

fn part_one(lines: &[String]) {
    let line_length = lines[0].len();
    let mut occurances_of_one: Vec<u16> = vec![0; line_length];

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                occurances_of_one[index] += 1;
            }
        }
    }

    for one in occurances_of_one.iter_mut() {
        if *one >= 500 {
            *one = 1;
        } else {
            *one = 0;
        }
    }

    // Convert to number
    let gamma_rate = occurances_of_one
        .iter()
        .fold(0, |acc, num| (acc << 1) + num);

    // Bitwise negate number
    let epsilon_rate = !gamma_rate;

    // Create a mask for clearing
    let mask: u16 = 0xFFFF >> (16 - line_length);

    // Clear first 4 bits to get the correct epsilon value
    let epsilon_rate = epsilon_rate & mask;

    let result = gamma_rate as u32 * epsilon_rate as u32;
    println!("{}", result);
}
