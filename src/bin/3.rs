use std::io::Result;

use aoc_2021::get_lines;

fn main() -> Result<()> {
    let lines = get_lines("inputs/3.txt")?;
    // part_one(&lines);
    part_two(&lines);

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

fn part_two(lines: &[String]) {
    let line_length = lines[0].len();
    let mut new_lines = lines.to_vec();
    for index in 0..line_length {
        new_lines = find_occurrences(&new_lines, index, true);
        if new_lines.len() == 1 {
            break;
        }
    }
    let oxygen_rating = binary_to_decimal(&new_lines[0]);

    let mut new_lines = lines.to_vec();
    for index in 0..line_length {
        new_lines = find_occurrences(&new_lines, index, false);
        if new_lines.len() == 1 {
            break;
        }
    }
    let co2_rating = binary_to_decimal(&new_lines[0]);

    let res = oxygen_rating * co2_rating;
    println!("{:?}", res);
}

fn find_occurrences(vec: &[String], index: usize, most_occurrent: bool) -> Vec<String> {
    let (occurrences_of_one, occurrences_of_zero): (Vec<_>, Vec<_>) = vec
        .iter()
        .partition(|x| x.chars().nth(index).unwrap() == '1');

    if occurrences_of_one.len() >= occurrences_of_zero.len() {
        if most_occurrent {
            occurrences_of_one
        } else {
            occurrences_of_zero
        }
    } else if most_occurrent {
        occurrences_of_zero
    } else {
        occurrences_of_one
    }
    .into_iter()
    .cloned()
    .collect()
}

fn binary_to_decimal(binary: &str) -> u32 {
    binary
        .chars()
        .fold(0, |acc, value| (acc << 1) + value.to_digit(2).unwrap())
}
