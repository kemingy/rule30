# Rule30

[![Rust](https://github.com/kemingy/rule30/actions/workflows/rust.yml/badge.svg)](https://github.com/kemingy/rule30/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/rule30.svg)](https://crates.io/crates/rule30)
[![docs.rs](https://docs.rs/rule30/badge.svg)](https://docs.rs/rule30)

Pseudo random number generator with cellular automaton rule 30.

## Features

- [x] `no_std`
- [x] Extend CA for better performance

## Usage

```rust
use rand_core::{RngCore, SeedableRng};
use rule30::ExtendedCA;

fn main() {
    let mut ca = ExtendedCA::seed_from_u64(42);
    println!("{}", ca.next_u64());
}
```

## Reference

- Wolfram [Rule30](https://reference.wolfram.com/language/tutorial/RandomNumberGeneration.html#830168163)
- Wolfram [ExtendedCA](https://reference.wolfram.com/language/tutorial/RandomNumberGeneration.html#18361715)
