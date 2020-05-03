use std::env;
use std::fs;
use std::process;

use bf;

fn main() {
    let mut data: Vec<u8> = vec![0; 1024];

    let filename = "hello.bf";

    let input = fs::read_to_string(String::from(filename)).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    let instructions = bf::parse(&input);

    println!("{:?}", instructions);

    let mut idx: isize = 0;

    bf::run(&instructions, data.as_mut_ptr(), &mut idx);
}
