use core::num;
use std::{fs, thread::current};

pub fn partone() {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;
    let data = fs::read_to_string("./data/day2_example.txt").expect("Unable to read file");

    for line in data.lines() {
        println!("Processing line: {}", line);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

}