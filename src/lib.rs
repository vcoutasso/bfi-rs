use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::num::Wrapping;

use Instructions::*;

/// The tuple enum variants hold a value that represents how many times the instruction should be repeated. This overcomes the overhead of repeating the same task over and over in the form of 'unit operations'
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instructions {
    /// Next pointer
    IncrementPointer(usize),
    /// Previous pointer
    DecrementPointer(usize),
    /// Increment data
    IncrementValue(usize),
    /// Decrement data
    DecrementValue(usize),
    /// Loop start
    BeginLoop,
    /// Loop end
    EndLoop,
    /// Read char from stdin
    ReadChar,
    /// Print value as char to stdout
    PrintChar,
    /// The folowing Instructions do not belong to bf and are here solely for optimization purposes
    ///
    /// Equivalent to [-] (set current cell to 0), but in one instruction
    SetZero,
}

/// Translates the code from a string of chars to a Vec of Instructions to be later matched against properly in run(). Returns a vector with the instructions in the order that they appear, but with some optimizations
pub fn parse(program: &str, opt_level: i32, verbose: bool) -> Vec<Instructions> {
    // Extract original instructions
    let instructions: Vec<_> = program
        .trim()
        .chars()
        .filter_map(|ch| match ch {
            '>' => Some(IncrementPointer(1)),
            '<' => Some(DecrementPointer(1)),
            '+' => Some(IncrementValue(1)),
            '-' => Some(DecrementValue(1)),
            '[' => Some(BeginLoop),
            ']' => Some(EndLoop),
            ',' => Some(ReadChar),
            '.' => Some(PrintChar),
            // Everything else is regarded as a comment
            _ => None,
        })
        .collect();

    if verbose {
        println!(
            "Original set of instructions contains {} operators",
            instructions.len()
        )
    }

    if opt_level > 0 {
        // Replaces all the occurrences of set_zero for the equivalent and more efficient Instruction::SetZero
        let set_zero = [BeginLoop, DecrementValue(1), EndLoop];

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

        // Keep track of how many repeated groupable instructions are close together for later simplification
        // e.g. ++ => IncrementValue(2)
        while i < instructions.len() {
            let mut acc = 1;

            // If groupable, create an equivalent instruction
            if groupable.contains(&instructions[i]) {
                while Some(&instructions[i]) == instructions.get(i + acc) {
                    acc += 1;
                }
            }
            // Check if the next 3 instructions are equivalent to SetZero and if so, use it instead
            else if opt_level > 1
            && instructions[i] == BeginLoop
            && i + set_zero.len() < instructions.len() // If the slice is not out of bounds
            && instructions[i..i+set_zero.len()] == set_zero
            // Check if it is equivalent to SetZero
            {
                optimized.push(SetZero);
                i += set_zero.len(); // Skip instructions we don't need anymore
                continue; // All done here, go to next
            }

            // Doesn't look very pretty, but it gets the job done
            optimized.push(match instructions[i] {
                IncrementPointer(_) => IncrementPointer(acc),
                DecrementPointer(_) => DecrementPointer(acc),
                IncrementValue(_) => IncrementValue(acc),
                DecrementValue(_) => DecrementValue(acc),
                _ => instructions[i],
            });
            i += acc;
        }
        if verbose {
            println!(
                "Optimized set of instructions contains {} operators",
                optimized.len()
            )
        }
        optimized
    } else {
        instructions
    }
}

/// Here's where the magic happens. With the course of action extracted with the parse() function, the only thing that is left to do is to take the appropriate action given an instruction
/// Returns the number of executed instructions and the index the pointer points at
pub fn run(inst: &[Instructions], memory: &mut [Wrapping<u8>], mut idx: usize) -> (usize, usize) {
    // Variable to keep track of how many instructions were performed
    let mut actions: usize = 0;
    // Counter
    let mut i = 0;

    // Indexes of begin loops to keep track of nested loops. Only used to fill jump
    let mut stack: Vec<usize> = Vec::new();
    // Vec with indexes of jumps to be made during execution (loops)
    let mut jump: Vec<usize> = vec![0; inst.len()];

    // This takes care of nested loops and how the interpreter should deal with them. jump will be filled with the indexes to perform the appropriate jumps at appropriate times
    for (i, instruction) in inst.iter().enumerate() {
        match instruction {
            BeginLoop => stack.push(i),
            EndLoop => {
                let index = stack.pop().expect("Could not find matching '['"); // Index of most recent loop
                jump[i] = index; // When code reaches the ith instruction, go to index and continue from there
                jump[index] = i; // When index is reached, go back to the start of the loop
            }
            _ => (),
        }
    }

    // Loop through all intructions
    while i < inst.len() {
        match inst[i] {
            // If idx is equal to the last position, return to the first
            IncrementPointer(qty) => {
                idx += qty;
                idx %= memory.len();
            }
            // If idx is equal to the first position, go to the last
            DecrementPointer(qty) => {
                if qty > idx {
                    idx = memory.len() - (qty - idx);
                } else {
                    idx -= qty;
                }
            }
            IncrementValue(qty) => {
                memory[idx] += Wrapping(qty as u8);
            }
            DecrementValue(qty) => {
                memory[idx] -= Wrapping(qty as u8);
            }
            BeginLoop => {
                if memory[idx] == Wrapping(0) {
                    i = jump[i];
                }
            }
            EndLoop => {
                if memory[idx] != Wrapping(0) {
                    i = jump[i];
                }
            }
            ReadChar => match io::stdin().bytes().next() {
                Some(res) => {
                    if let Ok(value) = res {
                        memory[idx] = Wrapping(value)
                    }
                }
                None => eprintln!("Could not read from stdin"),
            },
            PrintChar => print!("{}", char::from(memory[idx].0)),
            SetZero => memory[idx] = Wrapping(0),
        }
        actions += 1;
        i += 1;
    }
    (actions, idx)
}

/// Dump memory to file
pub fn dump_mem(memory: &[Wrapping<u8>], file: File, addr: usize) -> io::Result<()> {
    // Buffer the output
    let mut buf = BufWriter::new(file);

    // Quantity of bytes (different memory positions) per line
    let step = 12;

    buf.write_all(format!("Pointer pointing at address 0x{:04X}\n\n", addr).as_bytes())?;

    for i in (0..memory.len()).step_by(step) {
        buf.write_all(format!("0x{:04X}: \t", i).as_bytes())?;

        for value in memory.iter().skip(i).take(step) {
            buf.write_all(format!("0x{:02X} \t", value).as_bytes())?;
        }

        // Format last line properly if it is shorter than the previous ones
        if i + step > memory.len() {
            for _ in 0..(step - (memory.len() % step)) {
                buf.write_all(b"\t")?;
            }
        }

        for value in memory.iter().skip(i).take(step) {
            if value.0.is_ascii_graphic() {
                buf.write_all(format!("{}", value.0 as char).as_bytes())?;
            } else {
                buf.write_all(b".")?;
            }
        }

        buf.write_all(b"\n")?;
    }
    // Flush entire buffer at once
    buf.flush().unwrap();

    Ok(())
}

/// Dump instructions to file
pub fn dump_inst(instructions: &[Instructions], mut file: File) -> io::Result<()> {
    file.write_all(format!("{:#?}", instructions).as_bytes())?;

    Ok(())
}
