use std::{fs, io};
use std::io::Error;


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
                println!("{} and {} == 2020", i, x);
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
                    println!("{} {} and {} == 2020", i, x, z);
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
    while map.row+1 < map._max_row {
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
    while map.row+1 < map._max_row {
        map.move_pos(1, 1);
        if map.get_pos() == Some("#") {
            results.0 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r3, d1
    while map.row+1 < map._max_row {
        map.move_pos(1, 3);
        if map.get_pos() == Some("#") {
            results.1 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r5, d1
    while map.row+1 < map._max_row {
        map.move_pos(1, 5);
        if map.get_pos() == Some("#") {
            results.2 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r7, d1
    while map.row+1 < map._max_row {
        map.move_pos(1, 7);
        if map.get_pos() == Some("#") {
            results.3 += 1;
        }
    }
    map.row = 0;
    map.column = 0;

    // Slope of r1, d2
    while map.row+1 < map._max_row {
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
            _max_row: l.len(),
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

fn main() -> Result<(), std::io::Error> {

    let answer = day3_part2();
    println!("Day 2 answer is {:?}", answer);
    Ok(())
}
