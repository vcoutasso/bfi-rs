use std::env;
use std::fs;
use std::process;

use bf;

fn main() {
    let mut data: Vec<u8> = vec![0; 1024];

    let filename = "hello.bf";

    let program = fs::read_to_string(String::from(filename)).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    let instructions = bf::parse(&program);

    bf::run(&instructions, &mut data, &mut 0usize);
}
