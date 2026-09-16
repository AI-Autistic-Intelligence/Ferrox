use std::sync::RwLock;
use std::time::{Duration, Instant};
use rand::Rng;

/// Moving Target Defense (MTD) Cryptographic Key & Seed Shuffler
/// Periodically mutates internal signature seeds to invalidate black-box adversarial probing.
pub struct MovingTargetDefense {
    current_seed: RwLock<u64>,
    last_mutation: RwLock<Instant>,
    mutation_interval: Duration,
}

impl MovingTargetDefense {
    pub fn new(mutation_interval_secs: u64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            current_seed: RwLock::new(rng.gen()),
            last_mutation: RwLock::new(Instant::now()),
            mutation_interval: Duration::from_secs(mutation_interval_secs),
        }
    }

    /// Mutates seed if interval expired. Returns current active MTD seed.
    pub fn get_active_seed(&self) -> u64 {
        let now = Instant::now();
        let mut last = self.last_mutation.write().unwrap();

        if now.duration_since(*last) >= self.mutation_interval {
            let mut rng = rand::thread_rng();
            let mut seed = self.current_seed.write().unwrap();
            *seed = rng.gen();
            *last = now;
        }

        *self.current_seed.read().unwrap()
    }
}
