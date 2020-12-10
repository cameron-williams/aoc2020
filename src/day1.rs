/// Advent of Code Day 1
/// 
/// Input is many rows of a number per line.

use std::fs;
use std::io::Error;


/// Day 1 Part 1 is just finding the first two entries which sum to 2020 and return the multiple of them.
pub fn part_one() -> Result<Option<usize>, std::io::Error> {
    let list: Vec<usize> = fs::read_to_string("./day1_input.txt")?.split("\n").filter_map(|x| if let Ok(v) = x.parse::<usize>() {Some(v)} else {None}).collect();
    for i in &list {
        for x in &list {
            if i + x == 2020 {
                return Ok(Some(i*x))
            }
        }
    }
    Ok(None)
}

/// Day 1 Part 2 is the same as part 1 but instead finding the first 3 entries which sum to 2020, and returning the multiple of them.
pub fn part_two() -> Result<Option<usize>, std::io::Error> {
    let list: Vec<usize> = fs::read_to_string("./day1_input.txt")?.split("\n").filter_map(|x| if let Ok(v) = x.parse::<usize>() {Some(v)} else {None}).collect();
    for i in &list {
        for x in &list {
            for z in &list {
                if i + x + z == 2020 {
                    return Ok(Some(i*x*z))
                }
            }
        }
    }
    Ok(None)
}