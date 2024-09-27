use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_lines(str: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(str)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines.collect()
}

pub trait ToInt {
    fn to_int(&self) -> i32;
}

impl ToInt for String {
    fn to_int(&self) -> i32 {
        self.parse::<i32>().unwrap_or(0)
    }
}
