use rand::Rng;
use rand_distr::Normal;

use super::INITIAL_UNITS;

pub fn rand_unit_count(floor: usize) -> usize {
    let base = INITIAL_UNITS as f32 / 2.0;
    let base = base + (floor as f32 * 1.5);

    let mut rng = rand::thread_rng();
    let normal = Normal::new(base, base / 3.0).unwrap();

    let count = rng.sample(normal) as usize;

    if count < 1 {
        return 1;
    }

    count
}
