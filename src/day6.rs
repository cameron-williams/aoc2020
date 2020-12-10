/// Advent of Code Day 6
/// 
/// Takes an input of multiple groupings of strings of up to 26 letters a-z, strings are separated by \n in groupings, and groupings are separated by \n\n.
///
/// Both parts are split up into a few different iterators for readability, could chain them all together if you wanted.

use std::io::Error;
use std::fs;

use std::collections::HashSet;


/// Day 6 Part 1 needs to find how many unique letters are in each grouping of strings, each one only counting once.
pub fn part_one() -> Result<usize, Error> {
    let input = fs::read_to_string("./day6_input.txt")?;

    let lines: Vec<HashSet<char>> = input.split("\n\n").map(|l| {
        l.chars().filter(|c| *c != '\n').collect()
    }).collect();

    let total = lines.iter().fold(0, |tot, l| tot + l.len());
    Ok(total)
}

/// Day 6 Part 2 needs to identify which questions everyone answered yes to (all strings in group must share letter).
pub fn part_two() -> Result<usize, Error> {
    let input = fs::read_to_string("./day6_input.txt")?;

    let lines: Vec<HashSet<char>> = input.split("\n\n").filter_map(|l| {
        l.split("\n")
            .map(|ln| {
                ln.chars().collect()
            })
            .fold_first(|t, hs: HashSet<char>| {
                t.intersection(&hs).cloned().collect()
            })
    }).collect();

    let total = lines.iter().fold(0, |tot, l| tot + l.len());
    Ok(total)
}