//! Convenience re-export of common members.
//!
//! ```
//! use rule30::prelude::*;
//!
//! let mut ca = ExtendedCA::seed_from_u64(42);
//! println!("{}", ca.next_u64());
//! ```

// required dependencies
#[doc(no_inline)]
pub use rand_core::{RngCore, SeedableRng};

// internal modules
#[doc(no_inline)]
pub use crate::extended_ca::ExtendedCA;
#[doc(no_inline)]
pub use crate::rule30::Rule30;
