//! Pseudo random number generators with cellular automaton rule 30.
//!
//! ## Examples
//!
//! ```
//! use rand_core::{RngCore, SeedableRng};
//! use rule30::ExtendedCA;
//!
//! let mut ca = ExtendedCA::seed_from_u64(42);
//! println!("{}", ca.next_u64());
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![no_std]

mod extended_ca;
pub mod prelude;
mod rule30;

// Re-exports from rand_core
pub use rand_core::{RngCore, SeedableRng};

pub use self::extended_ca::ExtendedCA;
pub use self::rule30::Rule30;
