# Rule30

Pseudo random number generator with cellular automaton rule 30.

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
