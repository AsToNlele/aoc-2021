use aoc_2021::{get_lines, ToInt};
use std::io::Result;

fn main() -> Result<()> {
    let lines = get_lines("inputs/4.txt")?;
    part_one(&lines);

    Ok(())
}

fn part_one(lines: &[String]) {
    let mut lines_iter = lines.iter().peekable();

    let number_queue: Vec<_> = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.to_int())
        .collect();

    let mut bbs: Vec<BingoBoard> = vec![];

    while let Some(_) = lines_iter.next() {
        bbs.push(BingoBoard::load(&mut lines_iter));
    }

    for num in number_queue {
        for bingo_board1 in &mut bbs {
            let found = bingo_board1.set_found(num);
            if found {
                let solved = bingo_board1.check_solution();
                if solved {
                    let sum: i32 = bingo_board1.numbers.iter().fold(0, |acc, x| {
                        acc + x.iter().filter(|y| !y.found).map(|x| x.value).sum::<i32>()
                    });
                    println!("Part 1: {}", sum * num);
                    return;
                }
            }
        }
    }
}

#[derive(Debug)]
struct BingoNumber {
    value: i32,
    found: bool,
}

impl BingoNumber {
    fn new(value: i32) -> Self {
        Self {
            value,
            found: false,
        }
    }

    fn set_found(&mut self) {
        self.found = true;
    }
}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<Vec<BingoNumber>>,
}

impl BingoBoard {
    fn load<'a, I>(mut iter: I) -> Self
    where
        I: Iterator<Item = &'a String>,
    {
        let mut numbers: Vec<Vec<BingoNumber>> = vec![];
        let mut count = 0;
        while count < 5 {
            let line = iter.next().unwrap();
            if line.is_empty() {
                break;
            }
            let split: Vec<_> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .map(BingoNumber::new)
                .collect();
            if !split.is_empty() {
                numbers.push(split);
            }
            count += 1;
        }
        BingoBoard { numbers }
    }

    fn set_found(&mut self, found_number: i32) -> bool {
        self.numbers
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .find(|number| number.value == found_number)
            .map(|number| {
                number.found = true;
                true
            })
            .unwrap_or(false)
    }

    fn check_solution(&self) -> bool {
        let row_bingo = self
            .numbers
            .iter()
            .any(|row| row.iter().filter(|number| number.found).count() == self.numbers.len());

        let column_bingo = (0..self.numbers.len())
            .any(|j| self.numbers.iter().filter(|row| row[j].found).count() == self.numbers.len());

        row_bingo || column_bingo
    }
}
