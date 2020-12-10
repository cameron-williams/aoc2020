#![feature(iterator_fold_self)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), std::io::Error> {

    println!("Day 1, Part 1 answer is {:?}", day1::part_one());
    println!("Day 1, Part 2 answer is {:?}\n", day1::part_two());

    println!("Day 2, Part 1 answer is {:?}", day2::part_one());
    println!("Day 2, Part 2 answer is {:?}\n", day2::part_two());

    println!("Day 3, Part 1 answer is {:?}", day3::part_one());
    println!("Day 3, Part 2 answer is {:?}\n", day3::part_two());

    println!("Day 4, Part 1 answer is {:?}", day4::part_one());
    println!("Day 4, Part 2 answer is {:?}\n", day4::part_two());

    println!("Day 5, Part 1 answer is {:?}", day5::part_one());
    println!("Day 5, Part 2 answer is {:?}\n", day5::part_two());

    println!("Day 6, Part 1 answer is {:?}", day6::part_one());
    println!("Day 6, Part 2 answer is {:?}\n", day6::part_two());

    println!("Day 7, Part 1 answer is {:?}", day7::part_one());
    println!("Day 7, Part 2 answer is {:?}\n", day7::part_two());

    println!("Day 8, Part 1 answer is {:?}", day8::part_one());
    println!("Day 8, Part 2 answer is {:?}\n", day8::part_two());

    println!("Day 9, Part 1 answer is {:?}", day9::part_one());
    println!("Day 9, Part 2 answer is {:?}\n", day9::part_two());
    Ok(())
}
