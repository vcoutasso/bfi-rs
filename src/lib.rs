use std::collections::VecDeque;
use std::io::{self, Read};
use std::num::Wrapping;

use Instructions::*;

#[derive(Debug, PartialEq, Clone)]
// The tuple enum variants hold a value that represents how many times the instruction should be repeated. This overcomes the overhead of repeating the same task over and over in the form of 'unit operations'
pub enum Instructions {
    IncrementPointer(usize), // Next pointer
    DecrementPointer(usize), // Previous pointer
    IncrementValue(usize),   // value++
    DecrementValue(usize),   // value--
    BeginLoop,               // Loop start
    EndLoop,                 // Loop end
    ReadChar,                // Read char from stdin
    PrintChar,               // Print value as char to stdout
}

// Translates the code from a string of chars to a Vec of Instructions to be later matched against properly in run(). Returns a vector with the instructions in the order that they appear
pub fn parse(program: &str) -> Vec<Instructions> {
    // Raw instructions extracted from program
    let mut instructions: Vec<Instructions> = vec![];

    for ch in program.trim().chars() {
        match ch {
            '>' => instructions.push(IncrementPointer(1)),
            '<' => instructions.push(DecrementPointer(1)),
            '+' => instructions.push(IncrementValue(1)),
            '-' => instructions.push(DecrementValue(1)),
            '[' => instructions.push(BeginLoop),
            ']' => instructions.push(EndLoop),
            ',' => instructions.push(ReadChar),
            '.' => instructions.push(PrintChar),
            // Everything else is regarded as a comment
            _ => continue,
        }
    }

    // This vector contains all instructions in their optimized form (grouped)
    let mut optimized: Vec<Instructions> = vec![];
    // This slice represents the enum variants that can be grouped together
    let groupable = [
        IncrementPointer(1),
        DecrementPointer(1),
        IncrementValue(1),
        DecrementValue(1),
    ];
    // Counter
    let mut i = 0;

    while i < instructions.len() {
        let mut acc = 1;
        if groupable.contains(&instructions[i]) {
            for next_inst in instructions[i + 1..].iter() {
                if instructions[i] == *next_inst {
                    acc += 1;
                } else {
                    break;
                }
            }
        }

        // Doesn't look very pretty, but I couldn't find another way to do it so far
        match instructions[i] {
            IncrementPointer(_) => optimized.push(IncrementPointer(acc)),
            DecrementPointer(_) => optimized.push(DecrementPointer(acc)),
            IncrementValue(_) => optimized.push(IncrementValue(acc)),
            DecrementValue(_) => optimized.push(DecrementValue(acc)),
            _ => optimized.push(instructions[i].clone()),
        }
        i += acc;
    }
    optimized
}

// Here's where the magic happens. With the course of action extracted with the parse() function, the only thing that is left to do is to take the appropriate action given an instruction
pub fn run(inst: &[Instructions], data: &mut [Wrapping<u8>], mut idx: usize) -> usize {
    // Variable to keep track of how many instructions were performed
    let mut actions: usize = 0;
    // Counter
    let mut i = 0;

    // Indexes of begin loops to keep track of nested loops. Only used to fill jump
    let mut begin: VecDeque<usize> = VecDeque::new();
    // Vec with indexes of jumps to be made during execution (loops)
    let mut jump: Vec<usize> = vec![0; inst.len()];

    // This takes care of nested loops and how the interpreter should deal to them. jump will be filled with the indexes to perform the appropriate jumps at appropriate times
    for i in 0..inst.len() {
        match inst[i] {
            BeginLoop => begin.push_back(i),
            EndLoop => {
                let index = begin.pop_back().expect("Could not find matching '['"); // Index of most recent loop
                jump[i] = index; // When code reaches the ith instruction, go to index and continue from there
                jump[index] = i; // When index is reached, go back to the start of the loop
            }
            _ => continue,
        }
    }

    // Loop through all intructions
    while i < inst.len() {
        match inst[i] {
            // If idx is equal to the last position, return to the first
            IncrementPointer(qty) => {
                idx += qty;
                idx %= data.len();
                actions += qty - 1; // Add the equivalent amount of instructions followed minus one (the last one is added after the match statement)
            }
            // If idx is equal to the first position, go to the last
            DecrementPointer(qty) => {
                if qty > idx {
                    idx = data.len() - (qty - idx);
                } else {
                    idx -= qty;
                }
                actions += qty - 1;
            }
            IncrementValue(qty) => {
                data[idx] += Wrapping(qty as u8);
                actions += qty - 1;
            }
            DecrementValue(qty) => {
                data[idx] -= Wrapping(qty as u8);
                actions += qty - 1;
            }
            BeginLoop => {
                if data[idx] == Wrapping(0) {
                    i = jump[i];
                }
            }
            EndLoop => {
                if data[idx] != Wrapping(0) {
                    i = jump[i];
                }
            }
            ReadChar => match io::stdin().bytes().next() {
                Some(res) => {
                    if let Ok(value) = res {
                        data[idx] = Wrapping(value)
                    }
                }
                None => eprintln!("Could not read from stdin"),
            },
            PrintChar => print!("{}", char::from(data[idx].0)),
        }
        actions += 1;
        i += 1;
    }
    actions
}
