/// Advent of Code Day 8
/// 
/// Day 8 takes an input of multiple lines with a command on each in the format of {command} {+ || -}{int}. The command can either be nop (no operation), acc (add to accumulator), or jmp (jump to instruction).
/// The given instructions will be an infinite loop.

use std::io::Error;
use std::fs;


// Day 8 Part 1 problem is to find the value of acc before the first instance of any instruction being run twice.
pub fn part_one() -> Result<isize, Error> {
    let input = fs::read_to_string("./day8_input.txt")?;
    let instructions: Vec<(&str, isize, usize)> = input.split("\n").filter_map(|l| {
        let mut split = l.split(" ");
        if let (Some(instr), Some(int)) = (split.next(), split.next()) {
            match int.parse::<isize>() {
                Ok(int) => Some((instr, int, 0)),
                Err(_) => None
            }
        } else {
            None
        }
    }).collect();

    match run_instructions(instructions) {
        InstructionResult::Loops(val) => return Ok(val),
        _ => {Ok(0)},
    }
}

/// Day 8 Part 2 problem is finding the value of the accumulator when the program terminates after changing one nop->jmp or jmp->nop which fixes the infinite loop in the program.
pub fn part_two() -> Result<isize, Error> {
    let input = fs::read_to_string("./day8_input.txt")?;
    let instructions: Vec<(&str, isize, usize)> = input.split("\n").filter_map(|l| {
        let mut split = l.split(" ");
        if let (Some(instr), Some(int)) = (split.next(), split.next()) {
            match int.parse::<isize>() {
                Ok(int) => Some((instr, int, 0)),
                Err(_) => None
            }
        } else {
            None
        }
    }).collect();
    
    // Iterate each instruction, if it's nop/jmp flip it and see if the program terminates.
    for (i, instr) in instructions.iter().enumerate() {

        // If instruction is nop or jmp, flip it's instruction and test to see if the program terminates.
        if ["nop", "jmp"].contains(&instr.0) {
            let mut new_instructions = instructions.clone();
            new_instructions[i] = (
                match instr.0 {
                    "nop" => "jmp",
                    "jmp" => "nop",
                    _ => {""}
                },
                instr.1,
                0
            );
            if let InstructionResult::Terminates(val) = run_instructions(new_instructions) {
                return Ok(val)
            }
        }
        
    }
    Ok(0)
}

enum InstructionResult {
    Loops(isize),
    Terminates(isize),
    Errors
}

// Run given set of instructions, return if it terminates or loops, as well as the accumulator result when it does.
fn run_instructions(mut instructions: Vec<(&str, isize, usize)>) -> InstructionResult {
    let mut accumulator: isize = 0;
    let mut index: isize = 0;

    // When index > len of instructions, program has successfully terminated.
    while index < instructions.len() as isize {
        
        // Should never be negative, but if it is for some reason return an error.
        if index.is_negative() {
            return InstructionResult::Errors
        }

        // Match current instruction, instructions are in the format (instruction, int, count)
        if let Some(val) = instructions.get_mut(index as usize) {

            match val {

                // Acc instruction adds the value to the accumulator total
                ("acc", i, 0) => {
                    accumulator += *i;
                    index += 1;
                },

                // Jmp jumps forward i (isize) number of instructions.
                ("jmp", i, 0) => {
                    // catch jmp 0 loops
                    if *i == 0 {
                        return InstructionResult::Loops(accumulator)
                    }
                    index += *i;
                },

                // Nop is no operation, just move to next instruction.
                ("nop", _, 0) => {
                    index += 1;
                },

                // Break on first instance of .2 being > 0 (meaning an instruction has already been run)
                (_,_,_) => {
                    return InstructionResult::Loops(accumulator)
                }
            }
            // Increment instruction call count.
            val.2 += 1;
        } else {return InstructionResult::Errors}

    }
    InstructionResult::Terminates(accumulator)
}
