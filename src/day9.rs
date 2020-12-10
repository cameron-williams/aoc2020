/// Advent of Code Day 9
/// 
/// Day 9 takes a list of numbers starting with a 25 count preamble, after that each number should bet he sum of any two of the 25 previous numbers.

use std::io::Error;
use std::fs;


/// Part 1 is finding the first number in the input list that doesn't follow the rule.
pub fn part_one() -> Result<Option<usize>, Error> {
    let input = fs::read_to_string("./day9_input.txt")?;
    let vals: Vec<usize> = input.split("\n").filter_map(|v| {if let Ok(val) = v.parse::<usize>(){Some(val)}  else {None}}).collect();

    // Iterate values to find the first one that doesn't follow the preamble rule.
    for (i, v) in vals.iter().enumerate().filter(|(i, _)| {*i >= 25}) {

        // Get range of numbers to check against our current val
        let range: Vec<&usize> = vals[i-25..i].iter().filter(|x| {*x < v}).collect();

        let mut matches_rule = false;
        for val in &range {
            let oth = v - *val;
            if range.contains(&&oth) {
                matches_rule = true;
                break;
            }
        }
        if !matches_rule {
            return Ok(Some(*v));
        }
    }
    Ok(None)
}

/// Part 2 is finding a contiguous set of at least two numbers which add up to the number from part 1, then taking the smallest and largest number in that range and summing them.
pub fn part_two() -> Result<Option<usize>, Error> {
    let input = fs::read_to_string("./day9_input.txt")?;
    let vals: Vec<usize> = input.split("\n").filter_map(|v| {if let Ok(val) = v.parse::<usize>(){Some(val)}  else {None}}).collect();

    // Get our invalid number from part 1.
    let invalid_number = match part_one()? {
        Some(v) => v,
        None => return Ok(None)
    };

    // Iterate values to find a contiguous set adding up to invalid_number. Skip any values > than invalid number.
    for (i, v) in vals.iter().enumerate().filter(|(_, v)| {**v < invalid_number}) {
        
        let mut current_set: Vec<usize> = vec![v.clone()];

        // Iterate all numbers after current index and add to current set to see if it matches the criteria.
        for val in &vals[i+1..] {
            
            let tot: usize = current_set.iter().sum();
            if tot + val == invalid_number && current_set.len() >= 2{
                current_set.sort();
                return Ok(Some(current_set[0] + current_set[current_set.len()-1]));
            }
            else if tot + val < invalid_number {
                current_set.push(val.clone());
            } else {break}
                
        }
    }
    Ok(None)
}