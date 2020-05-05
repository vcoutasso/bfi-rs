use clap::{App, Arg};

use num_format::{Locale, ToFormattedString};

use std::num::Wrapping;
use std::time::Instant;
use std::{fs, process};

fn main() {
    let now = Instant::now();

    let matches = App::new("rust-bf")
        .version("0.2.0")
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
                .help("Quantity of bytes reserved during the execution of the program")
                .takes_value(true)
                .default_value("30000"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Prints info about the execution time and instructions count"),
        )
        .arg(
            Arg::with_name("dump_memory")
                .short("d")
                .long("dump")
                .help("Prints the state of the reserved memory after the execution"),
        )
        .arg(
            Arg::with_name("list_instructions")
                .short("l")
                .long("list")
                .help("Prints the instructions executed"),
        )
        .arg(
            Arg::with_name("optimization_level")
                .short("O")
                .long("optimization")
                .takes_value(true)
                .default_value("1")
                .possible_values(&["0", "1", "2"])
                .help("Optimization level"),
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

    // Unwrap is safe here because clap guarantees only valid values will be accepted
    let opt_level = matches
        .value_of("optimization_level")
        .unwrap()
        .parse()
        .unwrap();

    let instructions = bf::parse(&program, opt_level);

    let count_instructions = bf::run(&instructions, &mut data, 0usize);

    if matches.occurrences_of("verbose") > 0 {
        println!(
            "\nFinished {} instructions in {:.4}s",
            count_instructions.to_formatted_string(&Locale::pt),
            now.elapsed().as_secs_f32(),
        );
    }
    if matches.occurrences_of("list_instructions") > 0 {
        println!("\nList of instructions:\n{:?}", instructions);
    }
    if matches.occurrences_of("dump_memory") > 0 {
        println!("\nMemory dump:\n{:?}", data);
    }
}
