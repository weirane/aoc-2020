# Rust solutions for [Advent of Code 2020](https://adventofcode.com/2020)

Solution files are located in `./src/bin`, one for a day. Input files are in
`./input`. Files like `NN` are my input files; `NN-s*` are sample inputs.

## Run

```sh
cargo run [input-filename]...
```

Examples:
```console
$ cargo run 01
$ cargo run 01-s
$ cargo run 02-s 02
```

To run the binaries directly, do `cargo run --bin=${day?}`, where `$day` is one
of `{01..25}`. Input are read from stdin.
