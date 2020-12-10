/// Advent of Code Day 3
/// 
/// Taking an input of multiple lines which contain '.'s and '#'.
/// '.' represents an open space, '#' represents a tree

use std::io::Error;
use std::fs;

/// Part 1 returns the number of trees you would hit if you continued down the lines in a 3 right down 1 pattern.
pub fn part_one() -> Result<usize, Error> {
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

/// Part 2 returns the number of trees you would hit if you multipled the amount of trees you'd hit on the following slopes (right x, down x): r1d1, r3d1, r5d1, r7d1, r1d2 
pub fn part_two() -> Result<usize, Error> {
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