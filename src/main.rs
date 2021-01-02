use clap::{App, Arg};

use num_format::{Locale, ToFormattedString};

use std::fs::{self, File};
use std::path::Path;
use std::time::Instant;

fn main() {
    // Get current time to measure total time taken to finish the execution
    let now = Instant::now();

    // Info about the program and all possible options/flags
    let matches = App::new("bfi")
        .version("0.2.1")
        .author("Vinícius Couto <vcoutasso@gmail.com>")
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
                .help("Prints info about the execution time and instruction count"),
        )
        .arg(
            Arg::with_name("dump_memory")
                .long("dump_mem")
                .takes_value(true)
                .value_name("FILENAME")
                .help("Dumps the memory contents to file after execution finishes"),
        )
        .arg(
            Arg::with_name("dump_instructions")
                .long("dump_inst")
                .takes_value(true)
                .value_name("FILENAME")
                .help("Dumps the set of instructions to file after execution finishes"),
        )
        .arg(
            Arg::with_name("optimize")
                .short("O")
                .long("optimize")
                .help("Optimizes the set of instructions before running the program"),
        )
        .get_matches();

    // Quantity of reserved bytes
    let memory_amount: usize = matches
        .value_of("memory")
        .unwrap()
        .parse()
        .expect("Error parsing arguments");

    // value_of_os allows for unicode characters
    let filename = matches.value_of_os("filename").unwrap();

    // The all mighty memory that will be used during runtime
    let mut memory: Vec<u8> = vec![0; memory_amount];

    // Verbose flag
    let verbose = matches.occurrences_of("verbose") == 1;

    if verbose {
        println!("Reading contents of program file..")
    };

    // Original raw program string
    let program = fs::read_to_string(filename).expect("Error reading file");

    // Optimization flag
    let optimize = matches.occurrences_of("optimize") == 1;

    if verbose {
        println!("Parsing list of instructions..")
    }

    // List of raw instructions parsed from program
    let instructions = bfi::parse(&program, optimize, verbose);

    // Print a new line to separate verbose output from the program output
    if verbose {
        println!()
    }

    // Return values are amount of actions taken (instructions) and address that the pseudo_pointer is currently pointing at
    let (count_instructions, address) = bfi::run(&instructions, &mut memory, 0usize);

    // If flag verbose
    if verbose {
        println!(
            "\nFinished {} instructions in {:.4}s",
            count_instructions.to_formatted_string(&Locale::pt),
            now.elapsed().as_secs_f32(),
        );
    }

    // If option dump_instructions
    if matches.occurrences_of("dump_instructions") == 1 {
        let output_path = Path::new(matches.value_of("dump_instructions").unwrap());
        let file = File::create(output_path).expect("Could not create output file");

        match bfi::dump_inst(&instructions, file) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Could not dump instructions to file: {}", err);
            }
        }
    }

    // If option dump_memory
    if matches.occurrences_of("dump_memory") == 1 {
        let output_path = Path::new(matches.value_of("dump_memory").unwrap());
        let file = File::create(output_path).expect("Could not create output file");

        match bfi::dump_mem(&memory, file, address) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Could not dump memory contents to file: {}", err);
            }
        }
    }
}
