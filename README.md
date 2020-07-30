# bfi-rs

A simple yet efficient brainfuck interpreter written in Rust.

## Build instructions

Note: Rust toolchain required

``` sh
$ git clone https://github.com/vcoutasso/bfi-rs ; cd bfi-rs # Clone the git repository and cd into the directory
$ cargo build --release # Build the project in release mode. Omitting the --release flag will build in debug mode
```

## Usage 

``` sh
bfi [FLAGS] [OPTIONS] <FILENAME>
```

The executable can also be called by `cargo run --release` (the `--release` flag is optional but recommended for better performance). It expects a path to the brainfuck program file, which will be read and executed.

The level of optimization and the amount of memory reserved is configurable through their respective flags and bf will output to stdout by default.

## Implementation details and features

- Cells are unsigned 8-bit wrapping integers.
- The starting memory index is 0.
- Reserved memory is heap allocated and its size is fixed by the -m option.
- Trying to access an out of bounds address is defined behavior (wraps by default).
- The interpreter ships with a memory and instructions dumper accessible through their respective arguments.

## License

This software is free to use under the MIT License. See [this reference](https://choosealicense.com/licenses/mit/) for more information.
