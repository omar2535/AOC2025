use core::num;
use std::{fs, thread::current};

pub fn partone() {
    let data = fs::read_to_string("./data/day2.txt").expect("Unable to read file");
    let mut invalid_id_sum: u128 = 0;

    for line in data.lines() {
        let parts = line.split(",");
        for part in parts {
            println!("Checking range: {}", part);
            let start = part.split('-').collect::<Vec<&str>>()[0].parse::<u128>().unwrap();
            let end = part.split('-').collect::<Vec<&str>>()[1].parse::<u128>().unwrap();
            let invalid_ids = check_range_part1(start, end);
            for id in &invalid_ids {
                invalid_id_sum += id; 
            }
            println!("Invalid IDs between {} and {}: {:?}", start, end, invalid_ids);
        }
    }

    println!("Sum of all invalid IDs: {}", invalid_id_sum);
}

pub fn parttwo() {
    let data = fs::read_to_string("./data/day2.txt").expect("Unable to read file");
    let mut invalid_id_sum: u128 = 0;

    for line in data.lines() {
        let parts = line.split(",");
        for part in parts {
            println!("Checking range: {}", part);
            let start = part.split('-').collect::<Vec<&str>>()[0].parse::<u128>().unwrap();
            let end = part.split('-').collect::<Vec<&str>>()[1].parse::<u128>().unwrap();
            let invalid_ids = check_range_part2(start, end);
            for id in &invalid_ids {
                invalid_id_sum += id; 
            }
            println!("Invalid IDs between {} and {}: {:?}", start, end, invalid_ids);
        }
    }

    println!("Sum of all invalid IDs: {}", invalid_id_sum);
}

// Checks the range for any invalid IDs
// puts into a vector and returns it
fn check_range_part2(start: u128, end: u128) -> Vec<u128> {
    let mut range_vec: Vec<u128> = Vec::new();
    for i in start..=end {
        if is_invalid_id_part2(i) {
            range_vec.push(i);
        }
    }
    range_vec
}

// Checks the range for any invalid IDs
// puts into a vector and returns it
fn check_range_part1(start: u128, end: u128) -> Vec<u128> {
    let mut range_vec: Vec<u128> = Vec::new();
    for i in start..=end {
        if is_invalid_id_part1(i) {
            range_vec.push(i);
        }
    }
    range_vec
}

// Checks if the ID is invalid
// An ID is invalid if the two halves are equal
fn is_invalid_id_part1(id: u128) -> bool {
    let id_str: String = id.to_string();
    let len = id_str.len();

    // not even length so can't be invalid
    if len % 2 != 0 {
        return false;
    }
    
    // split in half and check the two halves
    let first_half = &id_str[0..len/2];
    let second_half = &id_str[len/2..len];
    if first_half == second_half {
        return true;
    }

    false
}

// Check if some sequence of digits is repeating at least twice
fn is_invalid_id_part2(id: u128) -> bool {
    let id_str: String = id.to_string();
    let len = id_str.len();

    for check_length in 1..=(len/2) {
        let substr = &id_str[0..check_length];
        if is_substring_repeating(substr, &id_str) {
            return true;
        }
    }

    false
}

fn is_substring_repeating(substr: &str, id_str: &str) -> bool {
    let mut index = 0;
    let substr_len = substr.len();
    let id_len = id_str.len();

    if substr_len == 0 || id_len % substr_len != 0 {
        return false;
    }

    while index + substr_len <= id_len {
        let checked_str = &id_str[index..index + substr_len];
        if checked_str != substr {
            return false;
        }
        index += substr_len;
    }

    true
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_substring_repeating() {
        assert_eq!(is_substring_repeating("12", "121212"), true);
        assert_eq!(is_substring_repeating("123", "123123123"), true);
        assert_eq!(is_substring_repeating("12", "121213"), false);
        assert_eq!(is_substring_repeating("123", "123124123"), false);
        assert_eq!(is_substring_repeating("1", "11111"), true);
        assert_eq!(is_substring_repeating("565", "565656"), false); 
        assert_eq!(is_substring_repeating("56", "565656"), true);
    }

    #[test]
    fn test_is_invalid_id_part2() {
        assert_eq!(is_invalid_id_part2(121212), true);
        assert_eq!(is_invalid_id_part2(123123123), true);
        assert_eq!(is_invalid_id_part2(121213), false);
        assert_eq!(is_invalid_id_part2(123124123), false);
        assert_eq!(is_invalid_id_part2(11111), true);
        assert_eq!(is_invalid_id_part2(565656), true);
        assert_eq!(is_invalid_id_part2(5656567), false);
    }
}