use std::io;

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

pub fn run(inst: &[Instructions], ptr: *mut u8, curr: &mut isize) {
    for (i, op) in inst.iter().enumerate() {
        //println!("{:?}", op);
        match op {
            Instructions::IncrementPointer => *curr += 1,
            Instructions::DecrementPointer => *curr -= 1,
            Instructions::IncrementValue => unsafe { *ptr.offset(*curr) += 1 },
            Instructions::DecrementValue => unsafe { *ptr.offset(*curr) -= 1 },
            Instructions::BeginLoop => continue,
            Instructions::EndLoop => continue,
            Instructions::ReadChar => continue,
            Instructions::PrintChar => unsafe { print!("{}", *ptr.offset(*curr) as char) },
        }
    }
}
