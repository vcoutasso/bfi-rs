use std::collections::VecDeque;
use std::io::{self, Read};
use std::num::Wrapping;

#[derive(Debug)]
pub enum Instructions {
    IncrementPointer, // Next pointer
    DecrementPointer, // Previous pointer
    IncrementValue,   // value++
    DecrementValue,   // value--
    BeginLoop,        // Loop start
    EndLoop,          // Loop end
    ReadChar,         // Read char from stdin
    PrintChar,        // Print value as char to stdout
}

// Translates the code from a string of chars to a Vec of Instructions to be later matched against properly in run(). Returns a vector with the instructions in the order that they appear
pub fn parse(program: &str) -> Vec<Instructions> {
    let mut op: Vec<Instructions> = vec![];

    for ch in program.chars() {
        match ch {
            '>' => op.push(Instructions::IncrementPointer),
            '<' => op.push(Instructions::DecrementPointer),
            '+' => op.push(Instructions::IncrementValue),
            '-' => op.push(Instructions::DecrementValue),
            '[' => op.push(Instructions::BeginLoop),
            ']' => op.push(Instructions::EndLoop),
            ',' => op.push(Instructions::ReadChar),
            '.' => op.push(Instructions::PrintChar),
            // Everything else is regarded as a comment
            _ => continue,
        }
    }
    op
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
            Instructions::BeginLoop => begin.push_back(i),
            Instructions::EndLoop => {
                let index = begin.pop_back().unwrap(); // Index of most recent loop
                jump[i] = index; // When code reach the ith instructions, go to index and continue from there
                jump[index] = i; // When index is reached, go back to the start of the loop
            }
            _ => continue,
        }
    }

    while i < inst.len() {
        match inst[i] {
            Instructions::IncrementPointer => {
                if idx == data.len() - 1 {
                    idx = 0
                } else {
                    idx += 1
                }
            }
            Instructions::DecrementPointer => {
                if idx == 0 {
                    idx = data.len() - 1
                } else {
                    idx -= 1
                }
            }
            Instructions::IncrementValue => data[idx] += Wrapping(1),
            Instructions::DecrementValue => data[idx] -= Wrapping(1),
            Instructions::BeginLoop => {
                if data[idx] == Wrapping(0) {
                    i = jump[i];
                }
            }
            Instructions::EndLoop => {
                if data[idx] != Wrapping(0) {
                    i = jump[i];
                }
            }
            Instructions::ReadChar => match io::stdin().bytes().next() {
                Some(res) => {
                    if let Ok(value) = res {
                        data[idx] = Wrapping(value)
                    }
                }
                None => eprintln!("Could not read from stdin"),
            },
            Instructions::PrintChar => print!("{}", char::from(data[idx].0)),
        }
        actions += 1;
        i += 1;
    }
    actions
}
