use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldRng {
    pub seed: u64,
    pub step: u64,
    #[serde(skip, default = "WorldRng::default_rng")]
    rng: ChaCha8Rng,
}

impl WorldRng {
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        Self { seed, step: 0, rng }
    }

    fn default_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(0)
    }

    pub fn reseed(&mut self, seed: u64, step: u64) {
        self.seed = seed;
        self.step = step;
        self.rng = ChaCha8Rng::seed_from_u64(seed.wrapping_add(step));
    }

    pub fn gen_range_f32(&mut self, min: f32, max: f32) -> f32 {
        self.step += 1;
        self.rng.gen_range(min..=max)
    }

    pub fn gen_range_u32(&mut self, min: u32, max: u32) -> u32 {
        self.step += 1;
        self.rng.gen_range(min..=max)
    }

    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.step += 1;
        self.rng.gen_bool(probability.clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_rng_reproducibility() {
        let mut rng1 = WorldRng::new(12345);
        let mut rng2 = WorldRng::new(12345);

        let val1: f32 = rng1.gen_range_f32(0.0, 100.0);
        let val2: f32 = rng2.gen_range_f32(0.0, 100.0);

        assert_eq!(val1, val2);
        assert_eq!(rng1.step, rng2.step);
    }
}
