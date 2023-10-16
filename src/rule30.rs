use rand_core::impls::{fill_bytes_via_next, next_u64_via_u32};
use rand_core::le::read_u32_into;
use rand_core::{RngCore, SeedableRng};

/// By default, the state bitmap size is 9.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rule30 {
    state: u32,
    skip: u32,
}

// Default multiplier for Rule 30.
const MULTIPLIER: u8 = 29;

impl Rule30 {
    pub fn new(state: u32, skip: u32) -> Self {
        Rule30::from_state_skip(state, skip)
    }

    #[inline]
    fn from_state_skip(state: u32, skip: u32) -> Self {
        let mut r = Rule30 { state, skip };
        for _ in 0..skip {
            r.step();
        }
        r
    }

    #[inline]
    fn step(&mut self) {
        self.state = (self.state >> 1) ^ (self.state | self.state << 1);
    }
}

impl SeedableRng for Rule30 {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 2];
        read_u32_into(&seed, &mut seed_u32);
        Rule30::from_state_skip(seed_u32[0], seed_u32[1])
    }
}

impl RngCore for Rule30 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let mut record: u32 = 0;
        for _ in 0..MULTIPLIER {
            record = (record ^ (self.state & 1)) << 1;
            self.step();
        }
        record
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
