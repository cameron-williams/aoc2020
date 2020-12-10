/// Advent of Code Day 5
/// 
/// Input is a list of strings which are 10 characters long. The first 7 are eiter F or B, and the last 3 are either R or L.
/// Using rows numbered 0 through 127, each letter tells you which half of a region the given seat is in: 0-127, first letter is F bringing us down to 0-63
/// 
/// Both day solutions can be simplified a bit, but wanted to break it out into a few steps instead of one giant chained interator.

use std::io::Error;
use std::fs;

/// Day 5 Part 1 is just getting the highest seat id of all input seats.
pub fn part_one() -> Result<usize, Error> {
    let input = fs::read_to_string("./day5_input.txt")?;
    let seats: Vec<SeatRange> = input.split("\n").map(|l| l.chars().fold(SeatRange::new(), |s, c| { s.eval(c) })).collect();

    let mut seat_ids: Vec<usize> = seats.iter().map(|s| s.id()).collect();
    seat_ids.sort();
    if let Some(v) = seat_ids.last() {
        return Ok(*v)
    }
    Ok(0)
}

/// Day 5 Part 2 is finding the missing seat id (our seat id) and returning it.
pub fn part_two() -> Result<usize, Error> {
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