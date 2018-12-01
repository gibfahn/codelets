# Advent of Code

Contains solutions for the 2018 [Advent of Code][] in [Rust][].

To see all answers, run

```bash
cargo run
```

The examples and problems are also implemented as tests. To run the tests:

```bash
# Run all tests
cargo test

# Don't hide stdout of passing tests
cargo test -- --nocapture --color always

# Run tests for a single day
cargo test -p one
```

To see the documentation and intros for all crates run:

```bash
cargo doc --document-private-items
cargo doc -p all --open
```

### Other solutions online

- [shepmaster](https://github.com/shepmaster/adventofcode2016)
- [carols10cents](https://github.com/carols10cents/adventofcode-rs)

[Advent of Code]: http://adventofcode.com/
[Rust]: https://www.rust-lang.org/
