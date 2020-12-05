use std::{fs, io};
use std::io::Error;
use std::collections::{HashMap, BTreeSet};

// Quick helper function to read a filename to a Vec<usize>.
fn read_to_vec_usize(filename: &str) -> Result<Vec<usize>, Error> {
    Ok(fs::read_to_string(filename)?.split("\n").into_iter().map(|x| x.parse::<usize>().unwrap()).collect())
}


/// Day 1 Solutions
/// 
/// Takes an input of a list of numbers, must find and return the first two entries
/// that sum up to 2020 and then return the multiple of them together.
fn day1_part1(list: Vec<usize>) -> Option<usize> {
    for i in &list {
        for x in &list {
            if i + x == 2020 {
                return Some(i*x)
            }
        }
    }
    None
}

fn day1_part2(list: Vec<usize>) -> Option<usize> {
    for i in &list {
        for x in &list {
            for z in &list {
                if i + x + z == 2020 {
                    return Some(i*x*z)
                }
            }
        }
    }
    None
}


/// Day 2 Solutions
/// 
/// Takes an input of a list of passwords and their password policy in the following format: 'n-m X: password' where X is a letter, n is the min amount of times and m is the max amount of times it can occur in the password.
/// Part1 function returns the amount of usable passwords (uses input file day2_input.txt).
fn day2_part1() -> Result<usize, Error> {
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

fn day2_part2() -> Result<usize, Error> {
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

// Takes a password/policy line from the day2 problem (n-m X: password) and extracts the useful information from it.
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



/// Day 3 Solutions
/// 
/// Taking an input of multiple lines which contain '.'s and '#'.
/// . represents an open space, # represents a tree
/// 
/// Part 1 returns the number of trees you would hit if you continued down the lines in a 3 right down 1 pattern.
fn day3_part1() -> Result<usize, Error> {
    let input = fs::read_to_string("./day3_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();


    let mut map = TobogganTracker::new(lines);
    let mut hit_trees = 0;
    while map.row < map._max_row {
        map.move_pos(1, 3);
        if map.get_pos() == Some("#") {
            hit_trees += 1;
        }
    }
    Ok(hit_trees)
}

fn day3_part2() -> Result<usize, Error> {
    let input = fs::read_to_string("./day3_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    let mut map = TobogganTracker::new(lines);

    let mut results: (usize, usize, usize, usize, usize) = (0, 0, 0, 0, 0);

    // Slope of r1,d1
    while map.row < map._max_row {
        map.move_pos(1, 1);
        if map.get_pos() == Some("#") {
            results.0 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r3, d1
    while map.row < map._max_row {
        map.move_pos(1, 3);
        if map.get_pos() == Some("#") {
            results.1 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r5, d1
    while map.row < map._max_row {
        map.move_pos(1, 5);
        if map.get_pos() == Some("#") {
            results.2 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r7, d1
    while map.row < map._max_row {
        map.move_pos(1, 7);
        if map.get_pos() == Some("#") {
            results.3 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r1, d2
    while map.row < map._max_row {
        map.move_pos(2, 1);
        if map.get_pos() == Some("#") {
            results.4 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    Ok(results.0 * results.1 * results.2 * results.3 * results.4)
}

/// Helper struct to make tracking the toboggan map easier.
struct TobogganTracker<'a> {
    map: Vec<&'a str>,
    row: usize,
    column: usize,
    _max_column: usize,
    _max_row: usize,
}

impl TobogganTracker<'_> {
    
    /// Create a new instance from a Vec of toboggan lines (&str).
    fn new(l: Vec<&str>) -> TobogganTracker {
        TobogganTracker {
            row: 0,
            column: 0,
            _max_column: l[0].len(),
            _max_row: l.len()-1,
            map: l,
        }
    }

    // Takes a row/column offset to move by.
    fn move_pos(&mut self, row: usize, col: usize) {
        let mut new_column = self.column + col;
        while new_column >= self._max_column {
            new_column -= self._max_column;
        }
        self.column = new_column;
        self.row += row;
    }

    // Get value at current row/column offset.
    fn get_pos(&self) -> Option<&str> {
        if let Some(row) = self.map.get(self.row) {
            return row.get(self.column..self.column+1);
        }
        None
    }
}


/// Day 4 Solutions
/// 
/// Input is in a batch file of key:value pairs separated by spaces or newlines. Separate passports are separated by blank lines.
/// 
/// There are eight fields, byr, iyr, eyr, ght, hcl, ecl, pid, cid
/// cid is optional, first problem is counting valid passports in the input file.
fn day4_part1() -> Result<usize, Error> {
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

fn day4_part2() -> Result<usize, Error> {
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



/// Day 5 Solutions
/// 
/// Input is a list of strings which are 10 characters long. The first 7 are eiter F or B, and the last 3 are either R or L.
/// Using rows numbered 0 through 127, each letter tells you which half of a region the given seat is in: 0-127, first letter is F bringing us down to 0-63
/// 
/// Both day solutions can be simplified a bit, but wanted to break it out into a few steps instead of one giant chained interator.
/// 
/// Day 1 is just getting the highest seat id of input.
fn day5_part1() -> Result<usize, Error> {
    let input = fs::read_to_string("./day5_input.txt")?;
    let seats: Vec<SeatRange> = input.split("\n").map(|l| l.chars().fold(SeatRange::new(), |s, c| { s.eval(c) })).collect();

    let mut seat_ids: Vec<usize> = seats.iter().map(|s| s.id()).collect();
    seat_ids.sort();
    if let Some(v) = seat_ids.last() {
        return Ok(*v)
    }
    Ok(0)
}

/// Day2 is finding the missing seat id (our seat id) and returning it.
fn day5_part2() -> Result<usize, Error> {
    let input = fs::read_to_string("./day5_input.txt")?;
    let seats: Vec<SeatRange> = input.split("\n").map(|l| l.chars().fold(SeatRange::new(), |s, c| { s.eval(c) })).collect();

    let mut seat_ids: Vec<usize> = seats.iter().map(|s| s.id()).collect();
    seat_ids.sort();

    for (i, id) in seat_ids.iter().enumerate() {
        if let Some(next_seat) = &seat_ids.get(i+1) {
            if *next_seat - id > 1 {
                return Ok(id+1)
            }
        }
    }
    Ok(0)
}

/// Helper struct for dealing with seat range col/rows. Wanted to make it foldable because why not?
#[derive(Debug)]
struct SeatRange {
    row: usize,
    col: usize,
    _row_start: usize,
    _row_end: usize,
    _col_start: usize,
    _col_end: usize,
}

impl SeatRange {

    fn new() -> SeatRange {
        SeatRange {
            row: 0,
            col: 0,
            _row_start: 0,
            _row_end: 127,
            _col_start: 0,
            _col_end: 7,
        }
    }

    fn eval(self, c: char) -> SeatRange {
        let mut seat_range = self;
        match c {
            'F' => {
                if (seat_range._row_end - seat_range._row_start) == 1 {
                    seat_range.row = seat_range._row_start;
                } else {
                    seat_range._row_end = seat_range._row_start + ((seat_range._row_end - seat_range._row_start)/2);    
                }
            },
            'B' => {
                if (seat_range._row_end - seat_range._row_start) == 1 {
                    seat_range.row = seat_range._row_end;
                } else {
                    seat_range._row_start = seat_range._row_start + ((seat_range._row_end - seat_range._row_start)/2)+1;    
                }
            },
            'L' => {
                if (seat_range._col_end - seat_range._col_start) == 1 {
                    seat_range.col = seat_range._col_start;
                } else {
                    seat_range._col_end = seat_range._col_start + ((seat_range._col_end - seat_range._col_start)/2);    
                }
            },
            'R' => {
                if (seat_range._col_end - seat_range._col_start) == 1 {
                    seat_range.col = seat_range._col_end;
                } else {
                    seat_range._col_start = seat_range._col_start + ((seat_range._col_end - seat_range._col_start)/2)+1;    
                }
            },
            _ => {}
        }
        seat_range
    }

    fn id(&self) -> usize {
        (self.row * 8) + self.col
    }
}

fn main() -> Result<(), std::io::Error> {

    let answer = day5_part2();

    println!("Day 5 answer is {:?}", answer);
    Ok(())
}
