use core::num;
use std::{fs, thread::current};

fn main() {
    partone();
}

pub fn partone() {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;
    let data = fs::read_to_string("./data/day1_example.txt").expect("Unable to read file");

    for line in data.lines() {
        let direction: &str = &line[0..1];
        let value: i32 = line[1..].parse().expect("Not a number!");

        println!("Direction: {}, Value: {}, Current Dial: {}", direction, value, dial);
        let (new_dial, turns) = get_new_dial_position(direction, dial, value);
        dial = new_dial;
        password += turns;

        println!("New Dial: {}, Turns: {}, Password: {}", dial, turns, password);
    }
    println!("Final Dial Position: {}", dial);
    println!("Password: {}", password);
}


pub fn parttwo() {
    let data = fs::read_to_string("./data/day2.txt").expect("Unable to read file");
    println!("{}", data);
}

pub fn get_new_dial_position(direction: &str, current_position: i32, value: i32) -> (i32, i32) {
    let remainder_value = value % 100;
    let mut new_position = current_position;
    let mut num_zeros_passed = 0;
    
    // calculate the number of times the dial has passed the 0 position
    if direction == "L" && current_position - value < 0 {
        println!("Calculating L direction zero passes");
        if current_position == 0 {
            num_zeros_passed = (value - current_position) / 100;
            println!("Current Position is 0");
        } else {
            num_zeros_passed = (value - current_position) / 100 + 1;
            println!("Current Position is not 0");
        }

        if current_position - (value % 100) == 0 {
            num_zeros_passed += 1;
        }
    } else if direction == "R" && current_position + value >= 100 {
        println!("Calculating R direction zero passes");
        num_zeros_passed = (current_position + value) / 100;
        if (current_position + value) % 100 == 0 {
            num_zeros_passed += 1;
        }
    }
    println!("Remainder Value: {}, Num Zeros Passed: {}", remainder_value, num_zeros_passed);

    if direction == "L" {
        if current_position - remainder_value < 0 {
            new_position = 100 + (current_position - remainder_value);
        } else {
            new_position = current_position - remainder_value;
        }
    } else if direction == "R" {
        if current_position + remainder_value >= 100 {
            new_position = (current_position + remainder_value) - 100;
        } else {
            new_position = current_position + remainder_value;
        }
    }

    (new_position, num_zeros_passed)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_dial_position() {
        assert_eq!(get_new_dial_position("L", 10, 20), (90, 1));
        assert_eq!(get_new_dial_position("R", 90, 20), (10, 1));
        assert_eq!(get_new_dial_position("L", 0, 150), (50, 1));
        assert_eq!(get_new_dial_position("L", 0, 100), (0, 1));
        assert_eq!(get_new_dial_position("R", 99, 150), (49, 2));
        assert_eq!(get_new_dial_position("L", 30, 549), (81, 6));
        assert_eq!(get_new_dial_position("R", 0, 100), (0, 1));
        assert_eq!(get_new_dial_position("R", 34, 66), (0, 1));
    }
}