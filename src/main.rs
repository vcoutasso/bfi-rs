use clap::{App, Arg};

use std::{fs, num::Wrapping, process};

fn main() {
    let matches = App::new("rust-bf")
        .author("Vinícius Couto <vinicouto12@gmail.com>")
        .about("A simple interpreter for the brainfuck programming language")
        .arg(
            Arg::with_name("filename")
                .value_name("FILENAME")
                .help("Path to the brainfuck program file to be executed")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("memory")
                .short("m")
                .long("memory")
                .value_name("BYTES")
                .help(
                    "The amount of bytes that will be reserved during the execution of the program",
                )
                .takes_value(true)
                .default_value("1024"),
        )
        .get_matches();

    let memory_amount: usize = match matches.value_of("memory").unwrap().parse() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error parsing arguments: {}", err);
            process::exit(1);
        }
    };

    let filename = matches.value_of("filename").unwrap();

    let mut data: Vec<Wrapping<u8>> = vec![Wrapping(0); memory_amount];

    let program = fs::read_to_string(String::from(filename)).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    let instructions = bf::parse(&program);

    bf::run(&instructions, &mut data, &mut 0usize);
}
