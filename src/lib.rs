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

pub fn bf_parse(program: &str) -> Vec<Instructions> {
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
            _ => continue,
        }
    }

    op
}
