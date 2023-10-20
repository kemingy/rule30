use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{RngCore, SeedableRng};

// Default cells number is 80 x 64 = 5120.
const SIZE: usize = 80;
const SKIP: usize = 4;
const INDEX: [usize; SKIP] = [16, 32, 48, 54];
const MASK: u64 = 0b0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001;

/// Pseudo-random number generator using Extended CA.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ExtendedCA {
    state: [u64; SIZE],
}

impl ExtendedCA {
    /// Create a new `ExtendedCA` instance with the given state.
    pub fn new(state: [u64; SIZE]) -> Self {
        ExtendedCA { state }
    }

    /// Extend the state array with 2 more states on both sides to make the evolution code simple.
    #[inline]
    fn extend_state(&self) -> [u64; SIZE + 4] {
        let mut extend = [0u64; SIZE + 4];
        extend[2..(SIZE + 2)].copy_from_slice(&self.state);
        extend[0] = self.state[SIZE - 2];
        extend[1] = self.state[SIZE - 1];
        extend[SIZE + 2] = self.state[0];
        extend[SIZE + 3] = self.state[1];
        extend
    }

    /// From the Wolfram reference on Extended CA:
    ///
    /// > "This generator uses a particular five-neighbor rule, so each new
    /// > cell depends on five nonadjacent cells from the previous step."
    ///
    /// It doesn't mention which particular five-neighbor rule is used. So I
    /// would choose 5 neighbors at the same position in the adjacent states.
    /// The evolving rule is similar to the Rule 30 (3 neighbors):
    ///
    /// `next_state[i] = state[i-2] | state[i-1] ^ state[i] ^ state[i+1] | state[i+2]`
    ///
    /// This requires further investigation.
    #[inline]
    fn step(&mut self) {
        let extend = self.extend_state();
        for i in 0..SIZE {
            self.state[i] =
                extend[i] | extend[i + 1] ^ extend[i + 2] ^ extend[i + 3] | extend[i + 4];
        }
    }
}

#[derive(Debug)]
pub struct ExtendedRngSeed(pub [u8; SIZE * 8]);

impl Default for ExtendedRngSeed {
    fn default() -> Self {
        ExtendedRngSeed([0u8; SIZE * 8])
    }
}

impl AsMut<[u8]> for ExtendedRngSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for ExtendedRngSeed {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl SeedableRng for ExtendedCA {
    type Seed = ExtendedRngSeed;

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; SIZE];
        read_u64_into(seed.as_ref(), &mut seed_u64);
        ExtendedCA::new(seed_u64)
    }
}

impl RngCore for ExtendedCA {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut num: u64 = 0;
        for i in INDEX.iter() {
            num = (num | (self.state[*i] & MASK)) << 1;
        }
        self.step();
        num
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

    use super::{ExtendedCA, ExtendedRngSeed, SIZE};

    #[test]
    fn test_extended_ca_construction() {
        let mut rng_u64 = ExtendedCA::seed_from_u64(42);
        assert_eq!(rng_u64.next_u64(), 6730243649896625144);
        assert_eq!(rng_u64.next_u64(), 7924083544289116156);

        let mut rng_u32 = ExtendedCA::seed_from_u64(42);
        assert_eq!(rng_u32.next_u32(), 4092244984);
        assert_eq!(rng_u32.next_u32(), 4225695740);

        let mut rng_gen = ExtendedCA::from_rng(&mut rng_u64).unwrap();
        assert_eq!(rng_gen.next_u64(), 18446744073709551584);

        let seed: [u8; SIZE * 8] = core::array::from_fn(|x| x as u8);
        let mut rng_seed = ExtendedCA::from_seed(ExtendedRngSeed(seed));
        assert_eq!(rng_seed.next_u64(), 4476646337808449056);

        let mut rng_double_check = ExtendedCA::seed_from_u64(42);
        assert_eq!(rng_double_check.next_u64(), 6730243649896625144);
    }
}
