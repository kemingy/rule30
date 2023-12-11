use rand_core::impls::{fill_bytes_via_next, next_u64_via_u32};
use rand_core::le::read_u32_into;
use rand_core::{RngCore, SeedableRng};

// Default cells number is 9 x 29 = 261.
const SIZE: usize = 9;
const VALID_LENGTH: usize = 29;
const MASK: u32 = 0b10;
const LEFT_MASK: u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
const RIGHT_MASK: u32 = 0b1;
const CLEAN_LEFT: u32 = 0b0011_1111_1111_1111_1111_1111_1111_1111;
const CLEAN_RIGHT: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1110;

/// Pseudo-random number generator using Rule 30.
///
/// Layout: `_x...y` where `x` is the second bit and `y` is the last bit.
///
/// There are 29 bits in the middle. `x` is the copy of the left neighbor's last bit,
/// and `y` is the copy of the right neighbor's second bit. These two bits are copied
/// for the evolution of the next state. `_` is omitted.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rule30 {
    state: [u32; SIZE],
    round: usize,
}

impl Rule30 {
    /// Create a new `Rule30` instance with the given state.
    pub fn new(state: [u32; SIZE]) -> Self {
        let round = (32 + SIZE - 1) / SIZE;
        let mut r = Rule30 { state, round };
        r.align();
        r
    }

    /// Align the second and last bits with neighbors to make the evolution code simple.
    #[inline]
    fn align(&mut self) {
        for i in 0..(SIZE - 1) {
            // align `y`
            self.state[i] =
                (self.state[i] & CLEAN_RIGHT) | ((self.state[i + 1] >> VALID_LENGTH) & RIGHT_MASK);
            // align `x`
            self.state[SIZE - 1 - i] = (self.state[SIZE - 1 - i] & CLEAN_LEFT)
                | (self.state[SIZE - 2 - i] << VALID_LENGTH) & LEFT_MASK;
        }
    }

    /// Rule 30: `next_state[i] = state[i-1] ^ (state[i] | state[i+1])`
    #[inline]
    fn step(&mut self) {
        for i in 0..SIZE {
            self.state[i] = (self.state[i] >> 1) ^ (self.state[i] | (self.state[i] << 1));
        }
        self.align();
    }
}

#[derive(Debug)]
pub struct Rule30RngSeed(pub [u8; SIZE * 4]);

impl Default for Rule30RngSeed {
    fn default() -> Self {
        Rule30RngSeed([0u8; SIZE * 4])
    }
}

impl AsMut<[u8]> for Rule30RngSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for Rule30RngSeed {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl SeedableRng for Rule30 {
    type Seed = Rule30RngSeed;

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; SIZE];
        read_u32_into(seed.as_ref(), &mut seed_u32);
        Rule30::new(seed_u32)
    }
}

impl RngCore for Rule30 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let mut record: u32 = 0;
        for _ in 0..self.round {
            for i in 0..SIZE {
                // assign the bit to the second bit of `record`
                record = (record | (self.state[i] & MASK)).rotate_left(1);
            }
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

#[cfg(test)]
mod tests {
    use rand_core::{RngCore, SeedableRng};

    use super::{Rule30, Rule30RngSeed, SIZE};

    #[test]
    fn test_rule30_construction() {
        let mut rng_u32 = Rule30::seed_from_u64(42);
        assert_eq!(rng_u32.next_u32(), 934717802);
        assert_eq!(rng_u32.next_u32(), 4047338106);

        let mut rng_u64 = Rule30::seed_from_u64(42);
        assert_eq!(rng_u64.next_u64(), 17383184802059299178);
        assert_eq!(rng_u64.next_u64(), 11370733897491177609);

        let mut rng_gen = Rule30::from_rng(&mut rng_u32).unwrap();
        assert_eq!(rng_gen.next_u32(), 1149652508);

        let seed: [u8; SIZE * 4] = core::array::from_fn(|x| x as u8);
        let mut rng_seed = Rule30::from_seed(Rule30RngSeed(seed));
        assert_eq!(rng_seed.next_u32(), 176394664);

        let mut rng_double_check = Rule30::seed_from_u64(42);
        assert_eq!(rng_double_check.next_u32(), 934717802);
    }
}
