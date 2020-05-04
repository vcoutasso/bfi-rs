# rust-bf

A simple and efficient brainfuck interpreter written in Rust.

## Build instructions

Rust toolchain required

``` sh
$ git clone https://github.com/vcoutasso/rust-bf ; cd rust-bf # Clone the git repository and cd into the directory
$ cargo build --release # Build the project in release mode. Omitting the --release flag will build in debug mode
```

## Implementation details

- The default integer overflow behavior is wrapping
- Reserved memory is static and constituted of an vector where each cell holds 1 byte (unsigned 8 bit integer) of data

## License

This software is free to use under the MIT License. See [this reference](https://choosealicense.com/licenses/mit/) for more information.
