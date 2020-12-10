/// Advent of Code Day 4
/// 
/// Input is in a batch file of key:value pairs separated by spaces or newlines. Separate passports are separated by blank lines.
/// There are eight possible fields on each passport: byr, iyr, eyr, ght, hcl, ecl, pid, cid

use std::io::Error;
use std::fs;

use std::collections::HashMap;

/// Day 4 Part 1 returns the amount of valid passports (have all fields but cid).
pub fn part_one() -> Result<usize, Error> {
    let input = fs::read_to_string("./day4_input.txt")?;
    
    let passports: Vec<HashMap<&str, &str>> = input.split("\n\n").map(|l| {
        l.split_whitespace().filter_map(|entry| {
            let mut split = entry.split(":");
            if let (Some(k), Some(v)) = (split.next(), split.next()) {
                Some((k, v))
            } else {None}
        }).collect()
    }).collect();

    let valid_passports = passports.iter().filter(|p| {
        p.keys().filter(|k| **k != "cid").count() >= 7
    }).count();
    
    Ok(valid_passports)
}


/// Day 4 Part 2 returns the amount of valid passports, which are passports which have the required fields and pass a validation test on them.
pub fn part_two() -> Result<usize, Error> {
    let input = fs::read_to_string("./day4_input.txt")?;
    
    let passports: Vec<HashMap<&str, &str>> = input.split("\n\n").map(|l| {
        l.split_whitespace().filter_map(|entry| {
            let mut split = entry.split(":");
            if let (Some(k), Some(v)) = (split.next(), split.next()) {
                Some((k, v))
            } else {None}
        }).collect()
    }).collect();

    let valid_passports = passports.iter().filter(|p| {
        validate_passport_fields(*p)
    }).count();

    Ok(valid_passports)
}

/// Checks if given passport has valid field values.
fn validate_passport_fields(p: &HashMap<&str, &str>) -> bool {
    // Check if passport has right number of min fields.
    if p.keys().filter(|k|**k != "cid").count() < 7 {
        return false
    }
    // Validate birth year field.
    if let Some(byr) = p.get("byr") {
        if let Ok(byr) = byr.parse::<usize>() {
            if byr < 1920 || byr > 2002 {
                return false
            }
        }
    } else {return false}

    // Validate issue year field.
    if let Some(iyr) = p.get("iyr") {
        if let Ok(iyr) = iyr.parse::<usize>() {
            if iyr < 2010 || iyr > 2020 {
                return false
            }
        } else {return false}
    } else {return false}

    // Validate Expiration year field.
    if let Some(eyr) = p.get("eyr") {
        if let Ok(eyr) = eyr.parse::<usize>() {
            if eyr < 2020 || eyr > 2030 {
                return false
            }
        } else {return false}
    } else {return false}

    // Validate height field.
    if let Some(hgt) = p.get("hgt") {
        // Handle inch height
        if hgt.contains("in") {
            if let Ok(hgt) = hgt[..2].parse::<usize>() {
                if hgt < 59 || hgt > 76 {
                    return false
                }
            } else {return false}
        }
        // Handle cm height
        else if hgt.contains("cm") {
            if let Ok(hgt) = hgt[..3].parse::<usize>() {
                if hgt < 150 || hgt > 193 {
                    return false
                }
            } else {return false}
        }
        else {
            return false
        }
    } else {return false}

    // Validate Hair color
    if let Some(hcl) = p.get("hcl") {
        // no regex, just manually set valid chars
        let valid_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        if hcl.len() != 7 || &hcl[..1] != "#" || hcl[1..].chars().filter(|c| valid_chars.contains(c)).count() != 6 {
            return false
        }
    }

    // Validate Eye color.
    if let Some(ecl) = p.get("ecl") {
        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_colors.contains(ecl) {
            return false
        }
    } else {return false}

    // Validate passport id.
    if let Some(pid) = p.get("pid") {
        if let Ok(_) = pid.parse::<usize>() {
            if pid.len() != 9 {
                return false
            }
        } else {return false}
    } else {return false}
    true
}

