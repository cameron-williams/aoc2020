/// Advent of Code Day 2
/// 
/// Input is rows of passwords and their policy in the following format: 'n-m X: password'

use std::io::Error;
use std::fs;

/// Day 2 Part 1 returns the amount of usable passwords in the input file, using the policy: X is a letter, n is the amount of times and m is the max amount of times that letter can occur in the password.
pub fn part_one() -> Result<usize, Error> {
    let input = fs::read_to_string("./day2_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    let valid_passwords = lines.iter().filter(|l| {
        if let Some(info) = extract_line_information(l) {
            let letter_count = info.0.chars().filter(|x| x == &info.1).count();
            if letter_count >= info.2 && letter_count <= info.3 {
                true
            } else {false}
        } else {
            false
        }
    }).count();
    Ok(valid_passwords)
}


/// Day 2 Part 2 changes the policy, where in 'n-m X: password' the letter x must occur at either position n or m in the password, and not both.
/// Returns the amount of usable passwords.
pub fn part_two() -> Result<usize, Error> {
    let input = fs::read_to_string("./day2_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    let valid_passwords = lines.iter().filter(|l| {
        if let Some(info) = extract_line_information(l) {
            let indexable: Vec<char> = info.0.split("").filter_map(|x| {
                if x.len() <= 0 {
                    None
                } else {
                    Some(x.parse::<char>().unwrap())
                }
            }).collect();
            let pos1 = indexable[info.2-1];
            let pos2 = indexable[info.3-1];
            match (pos1 == info.1, pos2 == info.1) {
                (true, true) => false,
                (false, false) => false,
                _ => true,
            }
        } else {false}
    }).count();
    Ok(valid_passwords)
}

// Takes a password/policy line (n-m X: password) and extracts the useful information from it.
// If line information is able to be extracted returns a tuple of (password, letter, min, max).
fn extract_line_information(s: &str) -> Option<(String, char, usize, usize)> {
    // Check if line is valid
    if s.len() <= 0 && !s.contains(":") {
        return None
    }

    let (mut min, mut max, mut letter) = (0, 0, ' ');
    let mut stage = 0;
    let mut current = String::new();
    for x in s.chars() {
        match x {
            '-' => {
                min = current.parse::<usize>().unwrap();
                current.clear();
            },
            ' ' => {
                if stage == 0 {
                    max = current.parse::<usize>().unwrap();
                    current.clear();
                    stage = 1;
                }
            },
            ':' => {
                letter = current.chars().next().unwrap();
                current.clear();
            },
            _ => {
                current.push(x);
            }
        }
    }
    Some((current, letter, min, max))
}