use std::io::{self, Read};
use std::num::Wrapping;

#[derive(Debug)]
pub enum Instructions {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    BeginLoop,
    EndLoop,
    ReadChar,
    PrintChar,
}

// Translates the code from a string of chars to a Vec of Instructions to be later matched against properly in run()
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
pub fn run(inst: &[Instructions], data: &mut [Wrapping<u8>], idx: &mut usize) -> usize {
    let mut it = inst.iter().enumerate();
    let mut actions: usize = 0;

    while let Some((i, op)) = it.next() {
        match op {
            Instructions::IncrementPointer => *idx += 1,
            Instructions::DecrementPointer => *idx -= 1,
            Instructions::IncrementValue => data[*idx] += Wrapping(1),
            Instructions::DecrementValue => data[*idx] -= Wrapping(1),
            // TODO: This approach does not work with nested loops!
            Instructions::BeginLoop => {
                let mut skip = 0;
                while data[*idx] != Wrapping(0) {
                    skip = run(&inst[i + 1..], data, idx);
                    actions += skip;
                }
                // Skip inner loop
                it.nth(skip);
            }
            Instructions::EndLoop => return actions + 1,
            Instructions::ReadChar => match io::stdin().bytes().next() {
                Some(res) => {
                    if let Ok(value) = res {
                        data[*idx] = Wrapping(value)
                    }
                }
                None => eprintln!("Could not read from stdin"),
            },
            Instructions::PrintChar => print!("{}", char::from(data[*idx].0)),
        }
        actions += 1;
    }
    actions
}
