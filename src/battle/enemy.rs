use rand::Rng;
use rand_distr::Normal;

use super::INITIAL_UNITS;

pub fn rand_unit_count(floor: usize) -> usize {
    let base =
        10.0 + (INITIAL_UNITS as f32) * (1.0 + floor as f32 / 10.0) - (INITIAL_UNITS / 2) as f32;

    let mut rng = rand::thread_rng();
    let normal = Normal::new(base, base / 3.0).unwrap();

    rng.sample(normal) as usize
}
