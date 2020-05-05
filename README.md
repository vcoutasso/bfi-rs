# bf-rs

A simple yet efficient brainfuck interpreter written in Rust.

## Build instructions

Note: Rust toolchain required

``` sh
$ git clone https://github.com/vcoutasso/rust-bf ; cd rust-bf # Clone the git repository and cd into the directory
$ cargo build --release # Build the project in release mode. Omitting the --release flag will build in debug mode
```

## Implementation details

- Cells are unsigned 8-bit wrapping integers.
- The starting memory index is 0.
- Reserved memory is heap allocated and its size is fixed by the -m option.
- Regarding optimization level (-O option), the options are:
  * 0: No optimizations performed.
  * 1: Repeated operations grouped when possible (e.g. +++ becomes a single operation that adds 3). This is the default.
  * 2: Same as Level 1 plus '[-]' replaced for the equivalent in a single instruction.
  
  As a side note, higher optimization level does not always mean lesser runtime. The parsing time to reach the optimized set of instructions _may_ be greater than the time shaved off.

## License

This software is free to use under the MIT License. See [this reference](https://choosealicense.com/licenses/mit/) for more information.
